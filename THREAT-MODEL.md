# Threat Model — http-error

Reference: STRIDE. Scope: the crate's public API surface. Trust boundary:
(1) bytes/inputs entering public constructors and parsers, (2) concurrent
callers sharing interior state. http-error is an in-process library — it opens
no sockets and inherits the embedding process's trust domain.

Purpose: HTTP error mapping (`http-errors`) — derive macro mapping error enums to status codes (re-export shim over `error-codes`)

## Assets

| ID | Asset | Exposed via |
|----|-------|-------------|
| A1 | status-code contract stability | hostile input, concurrent callers |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Residual risk |
|---|--------|----------|---------|------------|---------------|
| T1 | Divergence from `error-codes` mapping | Tampering | `re-export surface` | single source of truth in `error-codes`; shim adds nothing (compile-checked) | documented |

## Repudiation

The crate keeps no audit trail; attribution of calls to callers is out of
scope for an in-process library.

## Out of Scope

- Network transport security (the crate never opens sockets).
- Storage-host compromise: an attacker who controls the host can bypass all
  in-process mitigations.
- Denial of service via resource exhaustion of the host process beyond the
  bounds enforced above.

Reviewed: 2026-09-11
