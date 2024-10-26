use auth_service::{
    app_state::AppState,
    services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore,
        hashset_banned_token_store::HashsetBannedTokenStore,
    },
    utils::constants::prod,
    Application,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

    let app_state = AppState::new(user_store, banned_token_store, two_fa_code_store);
    let app = Application::build(app_state, prod::APP_ADDR)
        .await
        .expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
