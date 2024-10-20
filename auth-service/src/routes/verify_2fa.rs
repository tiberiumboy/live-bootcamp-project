use crate::routes::jwt::JWToken;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyToken {
    email: String,
    login_attempt_id: String,
    // This must be a string to includes leading zero's.
    // We will enforce the value numerical only
    #[serde(alias = "2FACode")]
    code: String,
}

impl VerifyToken {
    pub fn new(email: String, code: String) -> Self {
        Self {
            email,
            login_attempt_id: Uuid::new_v4().to_string(),
            code,
        }
    }
}

pub async fn verify_2fa(Json(input): Json<VerifyToken>) -> impl IntoResponse {
    /*
        allowed exception list:
        400: Invalid Input
        401: Authentication failed
        422: Unprocessable content
        500: Unexpected error (Should never happen)
    */

    dbg!(&input);
    let token = JWToken::validate(input.email, &input.login_attempt_id, &input.code).unwrap();
    dbg!(token);
    StatusCode::OK.into_response()
}
