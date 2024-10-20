use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{email::Email, password::Password},
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
    Json(login): Json<LoginRequest>,
) -> impl IntoResponse {
    let email = match Email::parse(&login.email) {
        Ok(email) => email,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let password = match Password::parse(&login.password) {
        Ok(password) => password,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let store = state.user_store.read().await;
    match store.validate_user(&email, &password).await {
        Ok(_) => {
            // validates user with email & password -> UserStore()?
            // create login_attempt_id & 2fa_code with TTL -> 2FA Code Store
            // Sends 2FA code -> Email services
            StatusCode::PARTIAL_CONTENT
        }
        Err(e) => {
            dbg!(e);
            StatusCode::UNAUTHORIZED
        }
    }
}
