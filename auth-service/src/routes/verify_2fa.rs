use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyToken {
    email: String,
    #[serde(rename = "loginAttemptId")]
    id: String,
    #[serde(rename = "2FACode")]
    code: String,
}

pub async fn verify_2fa(Json(_input): Json<VerifyToken>) -> impl IntoResponse {
    /*
        allowed exception list:
        400: Invalid Input
        401: Authentication failed
        422: Unprocessable content
        500: Unexpected error (Should never happen)
    */
    // match auth::validate_token(&input.token).await {
    //     Ok(_) => StatusCode::OK.into_response(),
    //     Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    // }
    StatusCode::OK.into_response()
}
