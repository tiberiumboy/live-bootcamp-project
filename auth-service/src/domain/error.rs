pub enum AuthAPIError {
    UserAlreadyExists,
    NotFound,
    InvalidCredentials,
    UnexpectedError,
    InvalidEmail,
    InvalidPassword,
}
