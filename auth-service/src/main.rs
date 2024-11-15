use auth_service::{
    app_state::AppState,
    domain::email::Email,
    services::{
        data_stores::{
            postgres_user_store::PostgresUserStore,
            redis_banned_token_store::RedisBannedTokenStore,
            redis_two_fa_code_store::RedisTwoFaCodeStore,
        },
        postmark_email_client::PostmarkEmailClient,
    },
    utils::{
        constants::{prod, DATABASE_URL, POSTMARK_AUTH_TOKEN, REDIS_HOST_NAME},
        tracing::init_tracing,
    },
    Application,
};
use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

fn configure_poskmark_email_client() -> PostmarkEmailClient {
    let http_client = Client::builder()
        .timeout(prod::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client!");
    let sender = prod::email_client::SENDER.to_owned();
    PostmarkEmailClient::new(
        prod::email_client::BASE_URL.to_owned(),
        Email::parse(Secret::new(sender)).expect("SENDER is improperly define in constants!"),
        POSTMARK_AUTH_TOKEN.to_owned(),
        http_client,
    )
}

async fn config_postgresql() -> PgPool {
    let pg_pool = Application::get_postgres_pool(&DATABASE_URL.expose_secret())
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
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing().expect("Fail to initialize tracing!");
    let pg_pool = config_postgresql().await;
    let redis_client = Arc::new(RwLock::new(configure_redis()));

    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_client.clone(),
    )));
    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFaCodeStore::new(redis_client.clone())));
    let email_client = Arc::new(configure_poskmark_email_client());

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
