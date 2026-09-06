# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

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
