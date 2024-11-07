use redis::{Commands, Connection, RedisResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    data_store::{TwoFACodeStore, TwoFACodeStoreError, TwoFARecord},
    email::Email,
    login_attempt_id::LoginAttemptId,
    two_fa_code::TwoFACode,
};

const TEN_MINUTE_TTL: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

pub type ARWRedisTwoFaCodeStoreType = Arc<RwLock<Connection>>;

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}

// what was the purpose for this?
#[derive(Serialize, Deserialize)]
pub struct TwoFaTuple(pub String, pub String);

pub struct RedisTwoFaCodeStore {
    client: ARWRedisTwoFaCodeStoreType,
}

impl RedisTwoFaCodeStore {
    pub fn new(client: ARWRedisTwoFaCodeStoreType) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFaCodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(&email);
        let instance = TwoFaTuple(id.as_ref().to_owned(), code.as_ref().to_string());
        let value =
            serde_json::to_string(&instance).map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        // The value should be the serialized 2FA tuple.
        // The expiration time should be set to TEN_MINUTES_IN_SECONDS.
        // Return TwoFACodeStoreError::UnexpectedError if casting fails or the call to set_ex fails.
        self.client
            .write()
            .await
            .set_ex(key, value, TEN_MINUTE_TTL)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(email);
        match self.client.write().await.del(key) {
            Ok(0) => Err(TwoFACodeStoreError::NotFound),
            Err(_) => Err(TwoFACodeStoreError::UnexpectedError),
            _ => Ok(()),
        }
    }

    async fn get_code(&self, email: &Email) -> Result<TwoFARecord, TwoFACodeStoreError> {
        let key = get_key(email);
        let mut db = self.client.write().await;
        let result: RedisResult<String> = db.get(key);
        match result {
            Ok(data) => {
                let code = serde_json::from_str(&data)
                    .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
                Ok(code)
            }
            Err(_) => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}
