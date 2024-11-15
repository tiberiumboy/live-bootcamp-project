use color_eyre::eyre::{Context, Result};
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
    #[tracing::instrument(name = "Add banned token to Redis", skip_all)]
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        let key = get_key(token);
        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .wrap_err("Fail to convert token_ttl_seconds into u64")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        self.client
            .write()
            .await
            .set_ex(key, true, ttl)
            .wrap_err("Fail to store banned token in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)
    }

    #[tracing::instrument(name = "Check banned token in Redis", skip_all)]
    async fn check_token(&self, token: &str) -> bool {
        let key = get_key(token);
        let mut db = self.client.write().await;
        db.exists(key).is_ok_and(|f: u32| f > 0)
    }
}
