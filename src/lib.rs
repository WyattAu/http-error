#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

//! Derive macro for mapping error enums to HTTP status codes.
//!
//! Provides `#[derive(HttpError)]` that auto-generates `status_code()`,
//! `error_code()`, and `IntoResponse` implementations.
//!
//! This crate is `no_std` + `alloc`: it only needs `String` for the
//! sanitized public error message.

extern crate alloc;

mod derive;
mod error;

pub use error::{ErrorCode, HttpError};
