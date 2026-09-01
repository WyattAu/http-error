use serde::Serialize;

/// Standard HTTP error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum ErrorCode {
    /// 400 Bad Request
    BadRequest,
    /// 401 Unauthorized
    Unauthorized,
    /// 403 Forbidden
    Forbidden,
    /// 404 Not Found
    NotFound,
    /// 409 Conflict
    Conflict,
    /// 422 Unprocessable Entity
    Validation,
    /// 429 Too Many Requests
    RateLimited,
    /// 500 Internal Server Error
    Internal,
}

impl ErrorCode {
    /// Get the HTTP status code.
    pub fn status_code(&self) -> u16 {
        match self {
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::Conflict => 409,
            Self::Validation => 422,
            Self::RateLimited => 429,
            Self::Internal => 500,
        }
    }
}

/// Trait for error types that can be converted to HTTP responses.
pub trait HttpError {
    /// Get the HTTP status code.
    fn status_code(&self) -> u16;
    /// Get the machine-readable error code.
    fn error_code(&self) -> &str;
    /// Get a sanitized error message (no internal details).
    fn public_message(&self) -> String;
}
