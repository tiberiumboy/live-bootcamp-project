use color_eyre::eyre::Context;
use redis::{Commands, Connection};
use secrecy::ExposeSecret;
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
    // TODO: Ask Bogdan about this?
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref().expose_secret())
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
    #[tracing::instrument(name = "Add 2FA code to Redis", skip_all)]
    async fn add_code(
        &mut self,
        email: Email,
        id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(&email);
        let instance = TwoFaTuple(
            id.as_ref().to_owned(),
            // TODO: Talk to Bogdan about this?
            code.as_ref().expose_secret().to_string(),
        );
        let value = serde_json::to_string(&instance)
            .wrap_err("Fail to serialize 2FA tuple")
            .map_err(TwoFACodeStoreError::UnexpectedError)?;

        // The value should be the serialized 2FA tuple.
        // The expiration time should be set to TEN_MINUTES_IN_SECONDS.
        // Return TwoFACodeStoreError::UnexpectedError if casting fails or the call to set_ex fails.
        self.client
            .write()
            .await
            .set_ex(key, value, TEN_MINUTE_TTL)
            .wrap_err("Fail to set 2FA code in Redis!")
            .map_err(TwoFACodeStoreError::UnexpectedError)
    }

    #[tracing::instrument(name = "Remove 2FA code from Redis", skip_all)]
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(email);
        let _: () = self
            .client
            .write()
            .await
            .del(key)
            .wrap_err("Fail to delete 2FA from Redis")
            .map_err(TwoFACodeStoreError::UnexpectedError)?;
        Ok(())
    }

    #[tracing::instrument(name = "Fetch 2FA code from Redis", skip_all)]
    async fn get_code(&self, email: &Email) -> Result<TwoFARecord, TwoFACodeStoreError> {
        let key = get_key(email);
        match self.client.write().await.get::<_, String>(key) {
            Ok(data) => {
                let code = serde_json::from_str(&data)
                    .wrap_err("Fail to deserialize 2FA struct")
                    .map_err(TwoFACodeStoreError::UnexpectedError)?;
                Ok(code)
            }
            Err(_) => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}
