use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UesrNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.iter().any(|e| e.0.eq(&user.get_email())) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        let email = user.get_email().to_owned();
        self.users.insert(email, user);
        Ok(())
    }

    // should we return a new clone copy, or a reference?
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        let user = self.users.iter().find(|e| e.0.eq(email));
        match user {
            Some((_, user)) => Ok(user.clone()),
            None => Err(UserStoreError::UesrNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password_match(password) == false {
            return Err(UserStoreError::InvalidCredentials);
        }
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
        let result = db.add_user(user);
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
        let result = db.add_user(user);
        assert_eq!(result.is_ok(), true);

        let result = db.get_user("test@test.com");
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
        let result = db.add_user(user);
        assert_eq!(result.is_ok(), true);

        let result = db.validate_user("test@test.com", "password");
        assert_eq!(result.is_err(), true);

        let result = db.validate_user("test@test.com", "password123!");
        assert_eq!(result.is_ok(), true);
    }
}
