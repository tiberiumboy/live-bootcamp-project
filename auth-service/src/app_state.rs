use crate::domain::data_store::{BannedTokenStore, UserStore};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub user_store: Arc<RwLock<dyn UserStore>>,
    pub banned_token_store: Arc<RwLock<dyn BannedTokenStore>>,
}

impl AppState {
    pub fn new(
        user_store: Arc<RwLock<dyn UserStore>>,
        banned_token_store: Arc<RwLock<dyn BannedTokenStore>>,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
        }
    }
}
