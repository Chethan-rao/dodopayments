#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum UserError {
    #[error("User InternalServerError")]
    InternalServerError,
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
}
