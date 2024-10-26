use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::domain::error::AuthAPIError;
use crate::domain::login_attempt_id::LoginAttemptId;
use crate::domain::two_fa_code::TwoFACode;
use crate::{
    app_state::AppState,
    domain::{email::Email, password::Password},
    utils::auth::generate_auth_cookie,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String, // Would be nice to use uuid?
}

impl LoginRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(login): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(&login.email) {
        Ok(email) => email,
        Err(_) => return (jar, Ok(StatusCode::BAD_REQUEST.into_response())),
    };

    let password = match Password::parse(&login.password) {
        Ok(pwd) => pwd,
        Err(_) => return (jar, Ok(StatusCode::BAD_REQUEST.into_response())),
    };

    let store = state.user_store.read().await;
    let user = match store.validate_user(&email, &password).await {
        Ok(user) => user,
        Err(_) => return (jar, Ok(StatusCode::UNAUTHORIZED.into_response())),
    };

    let result = match user.requires_2fa() {
        true => handle_2fa(&user.as_ref(), &state, jar).await,
        false => handle_no_2fa(&user.as_ref(), jar).await,
    };

    // a little hack to get this working. I'm sure there's a reason behind it?
    (result.0, Ok(result.1.into_response()))
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let id = LoginAttemptId::default();
    let code = TwoFACode::default();

    let mut two_fa_store = state.two_fa_code_store.write().await;
    // I wouldn't expect this to fail?
    if let Err(e) = two_fa_store.add_code(email.clone(), id.clone(), code).await {
        dbg!(e); // I'm curious what this could possibly fail?
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    // TODO: impl services that sends 2FA code to user's email

    let response = TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: id.as_ref().to_owned(),
    };

    (
        jar,
        Ok((
            StatusCode::PARTIAL_CONTENT,
            Json(LoginResponse::TwoFactorAuth(response)),
        )),
    )
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };
    let jar = jar.add(auth_cookie);
    (jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))
}
