# Requirements — http-error

Numbered, testable requirements. Every requirement maps to at least one named
test or doc-comment contract; security-relevant items cite threat-model rows.

Scope: HTTP error mapping (`http-errors`) — derive macro mapping error enums to status codes (re-export shim over `error-codes`)

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-HE-001 | Derive maps each variant to its declared status code; re-exports mirror `error-codes` semantics | MUST |
| REQ-HE-002 | The crate is a compile-time shim; no runtime behavior beyond `error-codes` | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-HE-100 | Macro output contains no panics on user input | MUST |

## Observability & API hygiene

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-HE-900 | All fallible public APIs return typed errors; production `unwrap`/`expect` is denied or explicitly justified with an invariant comment | MUST |
| REQ-HE-901 | Public items carry doc comments with runnable examples where practical | SHOULD |
