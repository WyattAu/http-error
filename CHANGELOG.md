# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

## [0.1.0] - 2026-09-11

### Fixed

- 22-gate quality audit pass: documentation completeness
  (README badges, REQUIREMENTS/THREAT-MODEL coverage) and
  feature-gated test hygiene.

### Changed
- This crate is now a thin re-export shim over `errcode` (package
  `error-codes`, the single status-mapping story: `ErrorCode` + `ErrCode` +
  `HttpError` + RFC 7807). `ErrorCode` variants, `status_code()`, the
  `HttpError` trait, and the serde shape are unchanged; `Unauthorized` /
  `Forbidden` / `Auth` / `Unavailable` variants plus `status()`, `as_str()`,
  `reason()`, and `type_uri()` come along via the re-export.

### Added
- `no_std` support (core + alloc; the crate never needed std).

### Changed
- Removed unused dependencies (`thiserror`, `http`, tokio dev-dep);
  `serde_json` moved to dev-dependencies; `serde` now builds without its
  std default feature.
- Fix two pre-existing clippy warnings in the proptest suite.

## [0.1.0]

### Added
- Derive macro for mapping error enums to HTTP status codes with IntoResponse.
- Not yet published to crates.io (crate name unavailable).
