use color_eyre::eyre::{eyre, Result};
use std::collections::HashMap;

use crate::domain::{
    data_store::{TwoFACodeStore, TwoFACodeStoreError, TwoFARecord},
    email::Email,
    login_attempt_id::LoginAttemptId,
    two_fa_code::TwoFACode,
};

#[derive(Default, Clone, Debug)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>, // not a good idea to use tuples instead of concrete type for data store?
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        if let Some(_) = self.codes.insert(email, (id, code)) {
            // if we received some, it means the key already exist instead, it updates the hashmap table, returning the old value back...
            // TODO: Discuss whether we need to handle this specific type of update or not?
            return Err(TwoFACodeStoreError::UnexpectedError(eyre!(
                "Key already exist!"
            )));
        }
        Ok(())
    }

    async fn get_code(&self, email: &Email) -> Result<TwoFARecord, TwoFACodeStoreError> {
        match self.codes.get(&email) {
            Some(code) => {
                let id = code.0.clone();
                let code = code.1.clone();
                let record = TwoFARecord { id, code };
                Ok(record)
            }
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(&email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        data_store::TwoFACodeStore, email::Email, login_attempt_id::LoginAttemptId,
        two_fa_code::TwoFACode,
    };

    use super::HashmapTwoFACodeStore;
    use secrecy::Secret;

    fn get_default_value() -> (Email, LoginAttemptId, TwoFACode) {
        // TODO: replace this with faker email address
        let random_email = "test@test.com".to_owned();
        let secret = Secret::new(random_email);
        let email = Email::parse(secret).expect("Unable to parse dummy email account");
        let id = LoginAttemptId::default();
        let code = TwoFACode::default();
        (email, id, code)
    }

    #[tokio::test]
    async fn add_code_should_succeed() {
        let mut db = HashmapTwoFACodeStore::default();

        let data = get_default_value();
        let result = db.add_code(data.0, data.1, data.2).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_code_should_succeed() {
        let mut db = HashmapTwoFACodeStore::default();
        let data = get_default_value();

        let result = db.add_code(data.0.clone(), data.1, data.2).await;
        assert!(result.is_ok());

        let data = db.get_code(&data.0).await;
        assert!(data.is_ok());
    }

    #[tokio::test]
    async fn remove_code_should_succeed() {
        let mut db = HashmapTwoFACodeStore::default();
        let data = get_default_value();

        let result = db.add_code(data.0.clone(), data.1, data.2).await;
        assert!(result.is_ok());

        let result = db.remove_code(&data.0).await;
        assert!(result.is_ok());
    }

    // TODO: impl expected failure case
}
