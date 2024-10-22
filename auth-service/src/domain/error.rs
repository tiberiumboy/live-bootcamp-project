use thiserror::Error;

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
