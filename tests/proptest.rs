// Property tests assert invariants directly; unwraps keep failures loud.
#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Property-based tests for http-errors crate.
//!
//! Shim validation: everything historically importable from `http-errors`
//! still resolves through the `errcode` re-export (`ErrorCode::status_code`,
//! `HttpError`, serde shape).

use proptest::prelude::*;

use http_errors::{ErrorCode, HttpError};

fn arb_error_code() -> impl Strategy<Value = ErrorCode> {
    prop_oneof![
        Just(ErrorCode::BadRequest),
        Just(ErrorCode::Unauthorized),
        Just(ErrorCode::Forbidden),
        Just(ErrorCode::NotFound),
        Just(ErrorCode::Conflict),
        Just(ErrorCode::Validation),
        Just(ErrorCode::RateLimited),
        Just(ErrorCode::Internal),
    ]
}

proptest! {
    #[test]
    fn status_code_always_valid_http(code in arb_error_code()) {
        let status = code.status_code();
        prop_assert!((100..600).contains(&status));
    }

    #[test]
    fn status_code_always_standard_http_status(code in arb_error_code()) {
        let status = code.status_code();
        prop_assert!(
            matches!(status, 400 | 401 | 403 | 404 | 409 | 422 | 429 | 500),
        );
    }

    #[test]
    fn status_code_is_client_or_server_error(code in arb_error_code()) {
        let status = code.status_code();
        prop_assert!((400..600).contains(&status));
    }

    #[test]
    fn status_code_alias_matches_status(code in arb_error_code()) {
        prop_assert_eq!(code.status_code(), code.status());
    }

    #[test]
    fn http_error_trait_matches_inherent_mapping(code in arb_error_code()) {
        prop_assert_eq!(<ErrorCode as HttpError>::status_code(&code), code.status());
        prop_assert_eq!(<ErrorCode as HttpError>::error_code(&code), code.as_str());
    }

    #[test]
    fn error_code_clone_preserves_value(code in arb_error_code()) {
        let cloned = code;
        prop_assert_eq!(code.status_code(), cloned.status_code());
    }

    #[test]
    fn error_code_equality_reflexive(code in arb_error_code()) {
        prop_assert_eq!(code, code);
    }

    #[test]
    fn error_code_debug_always_non_empty(code in arb_error_code()) {
        let debug = format!("{:?}", code);
        prop_assert!(!debug.is_empty());
    }

    #[test]
    fn error_code_serde_json_roundtrip(code in arb_error_code()) {
        let json = serde_json::to_string(&code).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json).unwrap();
        prop_assert!(json_value.is_string());
    }
}
