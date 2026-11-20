# Pattern adoption

Deprecated umbrella: this crate is now a thin re-export shim over
errcode, which owns the single status-mapping story.

| Pattern | Status | Evidence |
|---|---|---|
| P72 | partial | src/lib.rs:26 (re-exports the ErrorCode and HttpError contract with RFC 7807 Problem Details, stated at src/lib.rs:10; the mapping implementation lives in the errcode crate) |
| P74 | adopted | src/lib.rs:14 (migration table, inherent-method aliases kept, crate keeps publishing so existing dependents do not break) |
