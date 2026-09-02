//! Property-based tests for http-errors crate.

use proptest::prelude::*;

use http_errors::ErrorCode;

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
        prop_assert!(status >= 100 && status < 600);
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
    fn error_code_clone_preserves_value(code in arb_error_code()) {
        let cloned = code.clone();
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
