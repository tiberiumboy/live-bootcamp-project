use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub enum AuthAPIError {
    UserAlreadyExists,
    IncorrectCredentials,
    UnexpectedError,
    InvalidEmail,
    InvalidPassword,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_msg) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email input"),
            AuthAPIError::InvalidPassword => (StatusCode::BAD_REQUEST, "Invalid password input"),
            AuthAPIError::NotFound => (StatusCode::NOT_FOUND, "User is not found!"),
        };
        let body = Json(ErrorResponse {
            error: error_msg.to_owned(),
        });
        (status, body).into_response()
    }
}
