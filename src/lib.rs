#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

//! Derive macro for mapping error enums to HTTP status codes.
//!
//! > **Deprecated umbrella**: this crate is now a thin re-export shim over
//! > [`errcode`](https://github.com/WyattAu/errcode) (package `error-codes`),
//! > which is the single status-mapping story (`ErrorCode` + `ErrCode` +
//! > `HttpError` + RFC 7807 Problem Details). New code should depend on
//! > `errcode` directly. This crate keeps publishing so existing users don't
//! > break.
//!
//! # Migration
//!
//! | `http-errors` (old) | `errcode` (new) |
//! |------------------------|-----------------|
//! | `ErrorCode::BadRequest` … | unchanged (plus `Unauthorized`, `Forbidden`, `Auth`, `Unavailable`) |
//! | `code.status_code()` | unchanged (alias of `ErrorCode::status`) |
//! | `HttpError::status_code/error_code/public_message` | unchanged (`HttpError` now lives in `errcode`, implemented for `ErrorCode`) |
//!
//! The old `ErrorCode::status_code()` inherent method is kept as an alias of
//! [`status`](error_codes::ErrorCode::status), and `error_code()` strings are
//! available as [`as_str`](error_codes::ErrorCode::as_str).

pub use error_codes::*;
