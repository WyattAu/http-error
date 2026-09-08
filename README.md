# http-error

> Derive macro for mapping error enums to HTTP status codes.

> **Deprecated umbrella**: this crate is now a thin re-export shim over
> [`errcode`](https://github.com/WyattAu/errcode) (package `error-codes`),
> which is the single status-mapping story (`ErrorCode` + `ErrCode` +
> `HttpError` + RFC 7807 Problem Details). New code should depend on `errcode`
> directly. This crate keeps publishing so existing users don't break.

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)

## Quick Start

```rust
use http_errors::{HttpError, ErrorCode};

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

(`HttpError` and `ErrorCode` above now resolve through the `errcode`
re-export; the snippet is unchanged.)

## Migration to `errcode`

| `http-errors` (old) | `errcode` (new) |
|------------------------|-----------------|
| `ErrorCode::BadRequest` … | unchanged (plus `Unauthorized`, `Forbidden`, `Auth`, `Unavailable`) |
| `code.status_code()` | unchanged (alias of `ErrorCode::status`) |
| `HttpError::status_code/error_code/public_message` | unchanged trait, now in `errcode` (implemented for `ErrorCode`) |

## License

MIT OR Apache-2.0
