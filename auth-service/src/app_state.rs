use crate::domain::data_store::UserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub user_store: Arc<RwLock<dyn UserStore>>,
}

impl AppState {
    pub fn new(user_store: Arc<RwLock<dyn UserStore>>) -> Self {
        Self { user_store }
    }
}
