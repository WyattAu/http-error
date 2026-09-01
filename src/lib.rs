#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Derive macro for mapping error enums to HTTP status codes.
//!
//! Provides `#[derive(HttpError)]` that auto-generates `status_code()`,
//! `error_code()`, and `IntoResponse` implementations.

mod error;
mod derive;

pub use error::{HttpError, ErrorCode};
