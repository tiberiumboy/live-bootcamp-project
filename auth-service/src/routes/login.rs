use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::Deserialize;

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

pub async fn login(Json(login): Json<LoginRequest>) -> impl IntoResponse {
    dbg!(login);
    // validates user with email & password -> UserStore()?
    // create login_attempt_id & 2fa_code with TTL -> 2FA Code Store
    // Sends 2FA code -> Email services
    StatusCode::PARTIAL_CONTENT.into_response()
}
