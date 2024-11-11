use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use color_eyre::eyre::Report;
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
    UnexpectedError(#[source] Report),
    #[error("No JWT token was provided")]
    MissingToken, // no token was provided
    #[error("Oh look here, someone forging JWT. This incident will be logged and reported")]
    InvalidToken, // invalid JWT token was used
    #[error("Invalid data input: {0}")]
    InvalidData(String),
    #[error("Mismatch identification")]
    MismatchIdentification,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        log_error_chain(&self);
        let (status, error_msg) = match self {
            AuthAPIError::UserAlreadyExists => {
                (StatusCode::CONFLICT, "User already exist".to_owned())
            }
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_owned())
            }
            AuthAPIError::MissingToken => {
                (StatusCode::BAD_REQUEST, "Missing JWT Token!".to_owned())
            }
            AuthAPIError::MismatchIdentification => {
                (StatusCode::UNAUTHORIZED, "Mismatch identity".to_owned())
            }
            AuthAPIError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid JWT Token".to_owned())
            }
            AuthAPIError::UnexpectedError(report) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{report}"))
            }
            AuthAPIError::InvalidData(data) => {
                (StatusCode::BAD_REQUEST, format!("Invalid data: {data}"))
            }
        };
        let body = Json(ErrorResponse {
            error: error_msg.to_owned(),
        });
        (status, body).into_response()
    }
}

fn log_error_chain(e: &(dyn std::error::Error + 'static)) {
    let separator = "\n-------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }

    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}
