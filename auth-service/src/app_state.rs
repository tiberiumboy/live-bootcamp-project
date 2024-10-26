use crate::domain::data_store::{BannedTokenStore, TwoFACodeStore, UserStore};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub user_store: Arc<RwLock<dyn UserStore>>,
    pub banned_token_store: Arc<RwLock<dyn BannedTokenStore>>,
    pub two_fa_code_store: Arc<RwLock<dyn TwoFACodeStore>>,
}

impl AppState {
    pub fn new(
        user_store: Arc<RwLock<dyn UserStore>>,
        banned_token_store: Arc<RwLock<dyn BannedTokenStore>>,
        two_fa_code_store: Arc<RwLock<dyn TwoFACodeStore>>,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
            two_fa_code_store,
        }
    }
}
