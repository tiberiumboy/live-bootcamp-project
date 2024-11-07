use std::sync::Arc;
use tokio::sync::RwLock;

use redis::{Commands, Connection};

use crate::{
    domain::data_store::{BannedTokenStore, BannedTokenStoreError},
    utils::constants::TOKEN_TTL_SECONDS,
};

pub type ARWRedisClientType = Arc<RwLock<Connection>>;

const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}

pub struct RedisBannedTokenStore {
    client: ARWRedisClientType,
}

impl RedisBannedTokenStore {
    pub fn new(client: ARWRedisClientType) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        let key = get_key(token);
        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        self.client
            .write()
            .await
            .set_ex(key, true, ttl)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)
    }

    async fn check_token(&self, token: &str) -> bool {
        let key = get_key(token);
        let mut db = self.client.write().await;
        db.exists(key).is_ok_and(|f: u32| f > 0)
    }
}
