use auth_service::{
    app_state::AppState,
    services::{
        data_stores::{
            postgres_user_store::PostgresUserStore,
            redis_banned_token_store::RedisBannedTokenStore,
            redis_two_fa_code_store::RedisTwoFaCodeStore,
        },
        mock_email_client::MockEmailClient,
    },
    utils::constants::{prod, DATABASE_URL, REDIS_HOST_NAME},
    Application,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

async fn config_postgresql() -> PgPool {
    let pg_pool = Application::get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Fail to create Postgresql connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Fail to run migrations!");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    Application::get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client, is Redis installed?")
        .get_connection()
        .expect("Failed to get Redis connection! Is the port open and configured correctly?")
}

#[tokio::main]
async fn main() {
    let pg_pool = config_postgresql().await;
    let redis_client = Arc::new(RwLock::new(configure_redis()));

    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_client.clone(),
    )));
    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFaCodeStore::new(redis_client.clone())));
    let email_client = Arc::new(RwLock::new(MockEmailClient));

    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDR)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
