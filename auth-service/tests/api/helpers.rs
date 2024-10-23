use auth_service::{
    app_state::AppState,
    domain::data_store::BannedTokenStore,
    services::{
        hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore,
    },
    utils::constants::test,
    Application,
};
use reqwest::{cookie::Jar, Client};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: Client,
    pub banned_store: Arc<RwLock<dyn BannedTokenStore>>,
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

    pub fn get_random_email() -> String {
        format!("{}@example.com", Uuid::new_v4())
    }

    pub async fn new() -> Self {
        let banned_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let app_state = AppState::new(user_store, banned_store.clone());

        let app = Application::build(app_state, test::APP_ADDR)
            .await
            .expect("Failed to build app");
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());
        let cookie_jar = Arc::new(Jar::default());
        let http_client = Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
            http_client,
            banned_store,
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
