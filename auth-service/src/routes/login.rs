use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use secrecy::{ExposeSecret, Secret};
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
    email: Secret<String>,
    password: Secret<String>,
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
    pub fn new(email: Secret<String>, password: Secret<String>) -> Self {
        Self { email, password }
    }
}

#[tracing::instrument(name = "Login", skip_all)]
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(login): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email =
        Email::parse(login.email).map_err(|_| AuthAPIError::InvalidData("Email".to_owned()))?;
    let password = Password::parse(login.password)
        .map_err(|_| AuthAPIError::InvalidData("Password".to_owned()))?;
    let store = state.user_store.read().await;
    let user = store
        .validate_user(&email, &password)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    let result = match user.requires_2fa() {
        true => handle_2fa(&user.as_ref(), &state, jar).await,
        false => handle_no_2fa(&user.as_ref(), jar).await,
    };

    // a little hack to get this working. I'm sure there's a reason behind it?
    Ok((result.0, result.1.into_response()))
}

#[tracing::instrument(name = "Handle 2FA route", skip_all)]
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

    if let Err(e) = state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), id.clone(), code.clone())
        .await
    {
        // Need to discuss about this implementation -
        /*
           Currently theres nothing here that can stop us from updating existing value in our data store.
           If we tried to add a new entry with existing email account, the databse will simply update the original value, returning as an error message...?
           Also, currently, there's no implementation to clear the database of stored/temp values? E.g. Banned token have expiration date, but banned token store will keep that record forever.
        */
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    // TODO: impl services that sends 2FA code to user's email
    let body = format!(
        "Please use this code to log into the website: {}",
        code.as_ref().expose_secret()
    );
    if let Err(e) = state
        .email_client
        .read()
        .await
        .send_email(email, "Let's Get Rusty 2FA Code", &body)
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

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

#[tracing::instrument(name = "Handle No 2FA route", skip_all)]
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(e) => return (jar, Err(AuthAPIError::UnexpectedError(e))),
    };

    let jar = jar.add(auth_cookie);
    (jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))
}
