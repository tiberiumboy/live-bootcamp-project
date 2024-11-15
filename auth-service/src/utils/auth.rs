use super::constants::{JWT_COOKIE_NAME, JWT_SECRET, TOKEN_TTL_SECONDS};
use crate::domain::email::Email;
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Duration;
use chrono::Utc;
use color_eyre::eyre::{Context, ContextCompat, Result};
use jsonwebtoken::{
    decode, encode, errors::Error as JWTError, DecodingKey, EncodingKey, Validation,
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Task 4 requires me to update auth's claim to use secret, but encode needs to be able to serialize this input??
    pub exp: usize,
}

#[tracing::instrument(name = "Create new Json Web Token", skip_all)]
fn create_token(claim: Claims) -> Result<String> {
    encode(
        &jsonwebtoken::Header::default(),
        &claim, // I need to be able to serialize the claim???
        &EncodingKey::from_secret(JWT_SECRET.expose_secret().as_bytes()),
    )
    .wrap_err("Fail to generate Json Web Token")
}

#[tracing::instrument(name = "Create authentication cookie", skip_all)]
fn create_auth_cookie(token: String) -> Cookie<'static> {
    Cookie::build((JWT_COOKIE_NAME, token))
        .path("/") // apply cookie to all URLs on the server
        .http_only(true)
        .same_site(SameSite::Lax)
        .build()
}

#[tracing::instrument(name = "Generate new Json Web Token", skip_all)]
pub fn generate_auth_token(email: &Email) -> Result<String> {
    let delta =
        Duration::try_seconds(TOKEN_TTL_SECONDS).wrap_err("Fail to create 10 minute time delta")?;
    let exp = Utc::now()
        .checked_add_signed(delta)
        .wrap_err("Date is out of range")?
        .timestamp();

    let exp: usize = exp
        .try_into()
        .wrap_err("Unable to convert expiration type")?;

    let sub = email.as_ref().expose_secret().to_owned();
    let claims = Claims { sub, exp };
    create_token(claims)
}

#[tracing::instrument(name = "Generate authentication cookie", skip_all)]
pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>> {
    let token = generate_auth_token(email)?;
    Ok(create_auth_cookie(token))
}

#[tracing::instrument(name = "Validate Json Web Token", skip_all)]
pub async fn validate_token(token: &str) -> Result<Claims, JWTError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.expose_secret().as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::Secret;

    #[tokio::test]
    async fn test_generate_auth_cookie() {
        let input = "test@test.com".to_owned();
        let secret = Secret::new(input);
        let email = Email::parse(secret).unwrap();
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
        let account = "test@test.com".to_owned();
        let secret = Secret::new(account.clone());
        let email = Email::parse(secret).unwrap();
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
