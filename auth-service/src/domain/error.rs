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
    #[error("Invalid login id")]
    InvalidLoginId,
    #[error("Invalid 2FA Code")]
    Invalid2FACode,
    #[error("Mismatch identification")]
    MismatchIdentification,
}
