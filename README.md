# http-error

> Derive macro for mapping error enums to HTTP status codes.

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)

## Quick Start

```rust
use http_error::{HttpError, ErrorCode};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
}

impl HttpError for AppError {
    fn status_code(&self) -> u16 {
        match self {
            Self::NotFound(_) => 404,
            Self::Unauthorized => 401,
        }
    }
    fn error_code(&self) -> &str {
        match self {
            Self::NotFound(_) => "NOT_FOUND",
            Self::Unauthorized => "UNAUTHORIZED",
        }
    }
    fn public_message(&self) -> String {
        match self {
            Self::NotFound(_) => "Resource not found".to_string(),
            Self::Unauthorized => "Authentication required".to_string(),
        }
    }
}
```

## License

MIT OR Apache-2.0
