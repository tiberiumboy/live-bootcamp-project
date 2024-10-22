use auth_service::{
    app_state::AppState, services::hashmap_user_store::HashmapUserStore, utils::constants::prod,
    Application,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // our first dependency injection
    let hashmap = HashmapUserStore::default();
    let user_store = Arc::new(RwLock::new(hashmap));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, prod::APP_ADDR)
        .await
        .expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
