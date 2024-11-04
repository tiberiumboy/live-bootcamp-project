use auth_service::{
    app_state::AppState,
    services::{
        data_stores::{
            hashmap_two_fa_code_store::HashmapTwoFACodeStore,
            hashset_banned_token_store::HashsetBannedTokenStore,
            postgres_user_store::PostgresUserStore,
        },
        mock_email_client::MockEmailClient,
    },
    utils::constants::{prod, DATABASE_URL},
    Application,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pg_pool = config_postgresql().await;
    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
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
