use color_eyre::eyre::Report;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    email::Email, login_attempt_id::LoginAttemptId, password::Password, two_fa_code::TwoFACode,
    user::User,
};

#[derive(Debug, Error)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
                | (Self::UserNotFound, Self::UserNotFound)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                // This trick was used to help resolve partialEq compiliation error
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<User, UserStoreError>;
    async fn delete_user(&mut self, email: Email) -> Result<(), UserStoreError>;
}

#[derive(Debug, Error)]
pub enum BannedTokenStoreError {
    #[error("Token already exist!")]
    TokenExist,
    #[error("Token do not exist!")]
    TokenDoNotExist,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for BannedTokenStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::TokenDoNotExist, Self::TokenDoNotExist)
                | (Self::TokenExist, Self::TokenExist)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError>;
    async fn check_token(&self, token: &str) -> bool;
}

#[derive(Debug, Error)]
pub enum TwoFACodeStoreError {
    #[error("Login Attempt ID not found")]
    LoginAttemptIdNotFound,
    #[error("Not found")]
    NotFound,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for TwoFACodeStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::LoginAttemptIdNotFound, Self::LoginAttemptIdNotFound)
                | (Self::NotFound, Self::NotFound)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[derive(Deserialize)]
pub struct TwoFARecord {
    pub id: LoginAttemptId,
    pub code: TwoFACode,
}

#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync {
    async fn add_code(
        &mut self,
        email: Email,
        id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(&self, email: &Email) -> Result<TwoFARecord, TwoFACodeStoreError>;
}
