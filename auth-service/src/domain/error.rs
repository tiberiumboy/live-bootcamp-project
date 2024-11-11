use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Error)]
pub enum AuthAPIError {
    #[error("User already exist!")]
    UserAlreadyExists,
    #[error("Incorrect credentials was used")]
    IncorrectCredentials,
    #[error("Something terribly happen, may you qualify as a QA tester someday in the future")]
    UnexpectedError,
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Your password is too weak and insecure")]
    InvalidPassword,
    #[error("No JWT token was provided")]
    MissingToken, // no token was provided
    #[error("Oh look here, someone forging JWT. This incident will be logged and reported")]
    InvalidToken, // invalid JWT token was used
    #[error("Invalid login id")]
    InvalidLoginId,
    #[error("Invalid 2FA Code")]
    Invalid2FACode,
    #[error("Mismatch identification")]
    MismatchIdentification,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_msg) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid JWT Token"),
            AuthAPIError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email input"),
            AuthAPIError::InvalidPassword => (StatusCode::BAD_REQUEST, "Invalid password input"),
            AuthAPIError::InvalidLoginId => (StatusCode::BAD_REQUEST, "Invalid Login ID"),
            AuthAPIError::Invalid2FACode => (StatusCode::BAD_REQUEST, "Invalid 2FA Code"),
            AuthAPIError::MismatchIdentification => (StatusCode::UNAUTHORIZED, "Mismatch identity"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing JWT Token"),
        };
        let body = Json(ErrorResponse {
            error: error_msg.to_owned(),
        });
        (status, body).into_response()
    }
}
