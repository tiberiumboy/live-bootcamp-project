use auth_service::{
    app_state::{AppState, BannedTokenStoreType},
    domain::data_store::{BannedTokenStore, TwoFACodeStore},
    services::{
        data_stores::{
            hashmap_two_fa_code_store::HashmapTwoFACodeStore,
            postgres_user_store::PostgresUserStore,
            redis_banned_token_store::RedisBannedTokenStore,
        },
        mock_email_client::MockEmailClient,
    },
    utils::constants::{test, DATABASE_URL, REDIS_HOST_NAME},
    Application,
};
use reqwest::{cookie::Jar, Client};
use serde::Serialize;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Connection, Executor, PgConnection, PgPool,
};
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: Client,
    pub banned_store: BannedTokenStoreType,
    pub two_fa_code_store: Arc<RwLock<dyn TwoFACodeStore>>,
    db_name: String,
    clean_up_called: bool,
}

impl TestApp {
    async fn post<T: Serialize>(&self, url: &str, content: &T) -> reqwest::Response {
        self.http_client
            .post(url)
            .json(content)
            .send()
            .await
            .expect(&format!("Fail to post at url: {}", url))
    }

    async fn configure_database(db_conn_str: &str, db_name: &str) -> String {
        let connection = PgPoolOptions::new()
            .connect(db_conn_str)
            .await
            .expect("Fail to create Postgres connection pool.");

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
            .await
            .expect("Fail to create database");

        let db_conn_str = format!("{}/{}", db_conn_str, db_name);

        let connection = PgPoolOptions::new()
            .connect(&db_conn_str)
            .await
            .expect("Failed to create Postgres connection pool.");

        sqlx::migrate!()
            .run(&connection)
            .await
            .expect("Failed to migrate the database");

        db_conn_str
    }

    async fn configure_postgresql() -> (PgPool, String) {
        let postgresql_conn_url = DATABASE_URL.to_owned();

        let db_name = Uuid::new_v4().to_string();

        let postgresql_conn_url_with_db =
            Self::configure_database(&postgresql_conn_url, &db_name).await;

        // Create a new connection pool and return it
        (
            Application::get_postgres_pool(&postgresql_conn_url_with_db)
                .await
                .expect("Failed to create Postgres connection pool!"),
            db_name,
        )
    }

    async fn delete_database(db_name: &str) {
        let pgsql_conn_str = DATABASE_URL.to_owned();

        let conn_options = PgConnectOptions::from_str(&pgsql_conn_str)
            .expect("Failed to parse PostgreSQL connection string");

        let mut connection = PgConnection::connect_with(&conn_options)
            .await
            .expect("Fail to connect to PostgreSQL");

        // drop database and force close any active connections to the database.
        connection
            .execute(format!(r#"DROP DATABASE "{}" WITH (FORCE);"#, db_name).as_str())
            .await
            .expect("Failed to drop the database.");
    }

    pub async fn clean_up(&mut self) {
        TestApp::delete_database(&self.db_name).await;
        self.clean_up_called = true;
    }

    pub fn get_random_email() -> String {
        format!("{}@example.com", Uuid::new_v4())
    }

    pub async fn new() -> Self {
        let (pg_pool, db_name) = Self::configure_postgresql().await;
        let redis_conn = Application::get_redis_client(REDIS_HOST_NAME.to_owned())
            .expect("Failed to create Redis connection")
            .get_connection()
            .expect("Failed to get Redis connection");
        let redis_wrap = Arc::new(RwLock::new(redis_conn));
        let banned_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(redis_wrap)));
        let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let email_client = Arc::new(RwLock::new(MockEmailClient));
        let app_state = AppState::new(
            user_store,
            banned_store.clone(),
            two_fa_code_store.clone(),
            email_client, // do I need to include this in the struct?
        );
        let duration = Duration::from_secs(2);

        let app = Application::build(app_state, test::APP_ADDR)
            .await
            .expect("Failed to build app");
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());
        let cookie_jar = Arc::new(Jar::default());
        let http_client = Client::builder()
            .cookie_provider(cookie_jar.clone())
            .timeout(duration)
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
            http_client,
            banned_store,
            two_fa_code_store,
            db_name,
            clean_up_called: false,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to get root!")
    }

    pub async fn post_signup<T: Serialize>(&self, body: &T) -> reqwest::Response {
        self.post(&format!("{}/signup", &self.address), body).await
    }

    pub async fn post_login<T: Serialize>(&self, body: &T) -> reqwest::Response {
        self.post(&format!("{}/login", &self.address), body).await
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Fail to post logout request!")
    }

    pub async fn post_verify_2fa<T: Serialize>(&self, body: &T) -> reqwest::Response {
        self.post(&format!("{}/verify-2fa", &self.address), body)
            .await
    }

    pub async fn post_verify_token<T: Serialize>(&self, body: &T) -> reqwest::Response {
        self.post(&format!("{}/verify-token", &self.address), body)
            .await
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called {
            // TODO: Wish I could just call the cleanup function here but it's async... Shrug?
            panic!("Database was not clean up!");
        }
    }
}
