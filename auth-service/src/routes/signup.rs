use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    email: String,
    password: String,
    #[serde(alias = "requires2FA")]
    requires_2fa: bool,
}

pub async fn signup(Json(request): Json<SignupRequest>) -> impl IntoResponse {
    dbg!(request);
    // UserStore()
    // be sure to return json { "message": "User created successfully!" }
    StatusCode::CREATED.into_response()
}
