# AGENTS.md

## Scope

This crate is `api_clients_core`, the shared Rust library crate used by the
service-specific API clients in this workspace.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

`api_clients_core` owns common transport and error behavior:

- `Executor` for HTTP GET, POST with query strings, and POST with JSON body.
- `Executor::builder(api_endpoint)` for retry count, timeout, max RPS, and
  injected HTTP middleware client configuration.
- `ApiClientsError` and `ApiClientsResult` for common error handling.

It should stay service-agnostic. Service endpoint paths, request parameters, and
response models belong in the service crates.

## Public API Boundary

Treat these as public library contracts:

- `Executor`
- `Executor::builder`
- `Executor::exec_get`
- `Executor::exec_get_extra`
- `Executor::exec_post_qs`
- `Executor::exec_post_body`
- `ApiClientsError`
- `ApiClientsResult`

`ApiClientsError` is `#[non_exhaustive]`; downstream matches need wildcard arms
so new error variants can be added without breaking minor releases.

Do not change URL construction, retry defaults, timeout defaults, error
classification, rate-limit defaults, or response parsing semantics without
reviewing every service crate and downstream integration guidance.

## Important Gotchas

- `Executor` joins endpoint and path as `"{endpoint}/{path}"`. Endpoint strings
  normally must not end with `/`.
- `exec_get` sends an empty serde query object, which currently produces a URL
  with a trailing `?`.
- All successful responses are parsed as JSON. Non-JSON success bodies become
  `ApiClientsError::UnexpectedResponse`.
- 4xx responses map to `Client`; 5xx responses map to `Server`; other reqwest
  errors map through the reqwest conversions.
- Built executors rate-limit outbound HTTP attempts to 10 RPS by default. The
  limiter is smooth/no-burst and is installed after retry middleware so retries
  reserve their own slots.
- `with_max_rps(0)` is allowed and causes requests to wait indefinitely instead
  of sending.

## Downstream Integration Guidance

Final applications usually depend on a service crate instead of this crate
directly. Use `api_clients_core` directly only when:

- injecting a shared `Executor` into multiple generated clients
- configuring retry count, timeout, max RPS, or reqwest middleware once
- building a new service wrapper in this workspace

Application code should handle `ApiClientsError` explicitly and avoid
panic-driven response handling.

## Validation

For this crate:

```bash
cargo test -p api_clients_core
cargo +nightly fmt
cargo clippy -p api_clients_core --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p api_clients_core --no-deps
```

If transport behavior changes, also run every service crate test that can cover
request construction and response parsing.
