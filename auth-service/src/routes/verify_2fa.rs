use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::login_attempt_id::LoginAttemptId;
use crate::domain::two_fa_code::TwoFACode;
use crate::utils::auth::generate_auth_cookie;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyToken {
    email: String,
    #[serde(rename = "loginAttemptId")]
    id: String,
    #[serde(rename = "2FACode")]
    code: String,
}

pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(input): Json<VerifyToken>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = Email::parse(&input.email).map_err(|_| AuthAPIError::InvalidEmail)?;
    let id = LoginAttemptId::parse(input.id.clone()).map_err(|_| AuthAPIError::InvalidLoginId)?;
    let code = TwoFACode::parse(input.code.clone()).map_err(|_| AuthAPIError::Invalid2FACode)?;

    let mut two_fa_store = state.two_fa_code_store.write().await;
    let info = two_fa_store
        .get_code(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if info.id.ne(&id) || info.code.ne(&code) {
        return Err(AuthAPIError::MismatchIdentification);
    }

    // can't imagine this would break? maybe database error?
    if let Err(_) = two_fa_store.remove_code(&email).await {
        return Err(AuthAPIError::UnexpectedError);
    }

    let auth_cookie = generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;
    let jar = jar.add(auth_cookie);

    Ok((jar, StatusCode::OK.into_response()))
}
