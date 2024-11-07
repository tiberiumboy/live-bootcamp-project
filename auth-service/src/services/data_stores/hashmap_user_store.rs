use std::collections::HashMap;

use crate::domain::data_store::{UserStore, UserStoreError};
use crate::domain::{email::Email, password::Password, user::User};

#[derive(Default, Clone, Debug)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email = user.as_ref();
        if self.users.iter().any(|e| e.0.eq(email)) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let user = self.users.iter().find(|e| e.0.eq(email));
        match user {
            Some((_, user)) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<User, UserStoreError> {
        let user = self.get_user(email).await?;
        if user.password_match(&password) == false {
            return Err(UserStoreError::InvalidCredentials);
        }
        Ok(user)
    }

    async fn delete_user(&mut self, email: Email) -> Result<(), UserStoreError> {
        self.users.retain(|k, _| k.eq(&email));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;

    #[tokio::test]
    async fn test_add_user() {
        let email = "test@test.com";
        let password = "password123!";
        let result = User::parse(email, password, true);
        assert_eq!(result.is_ok(), true);
        let user = result.unwrap();
        let mut db = HashmapUserStore::default();
        let result = db.add_user(user).await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_get_user() {
        let email = "test@test.com";
        let password = "password123!";
        let result = User::parse(email, password, true);
        assert_eq!(result.is_ok(), true);

        let user = result.unwrap();
        let mut db = HashmapUserStore::default();
        let result = db.add_user(user.clone()).await;
        assert_eq!(result.is_ok(), true);

        let result = db.get_user(user.as_ref()).await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let email = "test@test.com";
        let password = "password123!";
        let result = User::parse(email, password, true);
        assert_eq!(result.is_ok(), true);

        let user = result.unwrap();
        let mut db = HashmapUserStore::default();
        let result = db.add_user(user.clone()).await;
        assert_eq!(result.is_ok(), true);

        let result = db.validate_user(user.as_ref(), user.as_ref()).await;
        assert_eq!(result.is_ok(), true);
    }
}
