use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::domain::error::AuthAPIError;
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

impl LoginRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

// somewhere here - I'd like to send a JWT as a response back,
// but seeing how we're implementing a two form factor authentication - I need to wait for the class to catch up on this before sending JWT response.
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
        Ok(password) => password,
        Err(_) => return (jar, Ok(StatusCode::BAD_REQUEST.into_response())),
    };

    let auth_cookie = generate_auth_cookie(&email);
    if auth_cookie.is_err() {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    let updated_jar = jar.add(auth_cookie.unwrap());

    let store = state.user_store.read().await;
    // validates user with email & password -> UserStore()?
    let status_code = match store.validate_user(&email, &password).await {
        Ok(account) => {
            // create login_attempt_id & 2fa_code with TTL -> 2FA Code Store
            // Sends 2FA code -> Email services
            match account.requires_2fa() {
                true => StatusCode::PARTIAL_CONTENT,
                false => StatusCode::OK,
            }
        }
        Err(_) => StatusCode::UNAUTHORIZED,
    };

    (updated_jar, Ok(status_code.into_response()))
}
