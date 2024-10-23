use std::collections::HashSet;

use crate::domain::data_store::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default, Debug, Clone)]
pub struct HashsetBannedTokenStore {
    // one thing that's concerning me is that there's no way to clean up this list after a few time pass.
    // if we assume that the token has indeed past it's expiration date, then we should purge those token from this list.
    pub blacklist: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        if self.blacklist.insert(token.to_owned()) {
            Ok(())
        } else {
            Err(BannedTokenStoreError::TokenExist)
        }
    }

    async fn check_token(&self, token: &str) -> bool {
        self.blacklist.get(token).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token_pass() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "token";

        // send token to the list
        let result = store.add_token(token).await;
        // our result should return Ok(())
        assert!(result.is_ok());
        // our token should exist in the database collection
        assert!(store.check_token(token).await);
    }

    #[tokio::test]
    async fn adding_duplicated_token_should_fail_test() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "token";

        assert!(store.add_token(token).await.is_ok());
        assert!(store.add_token(token).await.is_err()); // should report an error stating the token already exist.
    }

    #[tokio::test]
    async fn valid_token_should_pass_checks() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "token";

        assert!(store.add_token(token).await.is_ok());
        assert!(store.check_token(token).await);
    }

    #[tokio::test]
    async fn check_empty_store_should_fail() {
        let store = HashsetBannedTokenStore::default();
        let token = "token";
        assert_eq!(store.check_token(token).await, false);
    }

    #[tokio::test]
    async fn token_not_in_list_should_return_false() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "token";
        let search = "token1";
        assert!(store.add_token(token).await.is_ok());
        assert_eq!(store.check_token(search).await, false);
    }
}
