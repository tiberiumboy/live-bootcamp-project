use super::constants::{JWT_COOKIE_NAME, JWT_SECRET, TOKEN_TTL_SECONDS};
use crate::domain::email::Email;

use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error as JWTError, DecodingKey, EncodingKey, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug)]
pub enum GenerateTokenError {
    TokenError(JWTError),
    UnexpectedError,
}

fn create_token(claim: &Claim) -> Result<String, JWTError> {
    encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

fn generate_auth_token(email: &Email) -> Result<String, GenerateTokenError> {
    let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS)
        .ok_or(GenerateTokenError::UnexpectedError)?;

    let exp = Utc::now()
        .checked_add_signed(delta)
        .ok_or(GenerateTokenError::UnexpectedError)?
        .timestamp();

    let exp: usize = exp
        .try_into()
        .map_err(|_| GenerateTokenError::UnexpectedError)?;

    let sub = email.as_ref().to_owned();
    let claims = Claim { sub, exp };

    create_token(&claims).map_err(GenerateTokenError::TokenError)
}

fn create_auth_cookie(token: String) -> Cookie<'static> {
    let cookie = Cookie::build((JWT_COOKIE_NAME, token))
        .path("/") // apply cookie to all URLs on the server
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    cookie
}

pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>, GenerateTokenError> {
    let token = generate_auth_token(email)?;
    Ok(create_auth_cookie(token))
}

pub async fn validate_token(token: &str) -> Result<Claim, JWTError> {
    decode::<Claim>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_auth_cookie() {
        let email = Email::parse("test@test.com").unwrap();
        let cookie = generate_auth_cookie(&email).unwrap();
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value().split('.').count(), 3);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_create_auth_cookie() {
        let token = "test_token".to_owned();
        let cookie = create_auth_cookie(token.clone());
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value(), token);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_validate_token_with_valid_token() {
        let account = "test@test.com";
        let email = Email::parse(&account).unwrap();
        let token = generate_auth_token(&email).unwrap();
        let result = validate_token(&token).await;

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.sub, account);

        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::try_minutes(9).expect("valid duration"))
            .expect("Unable to check added signed")
            .timestamp() as usize;

        assert!(result.exp > exp)
    }

    #[tokio::test]
    async fn test_validate_token_with_invalid_token() {
        let token = "invalid_token";
        let result = validate_token(token).await;
        assert!(result.is_err());
    }
}
