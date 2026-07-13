# AGENTS.md

## Scope

This crate is `bidask_api_client`, an unsupported legacy Rust library crate for
Bidask.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate currently contains a thin typed client:

- `BidaskApiClient::builder().build()?`
- `client.api.exec(&Request::...)`
- request enum in `api/request.rs`
- response enum and models in `api/response.rs` and `api/types.rs`

The crate is not supported for final-app integration. Do not recommend it to
downstream users, document it as available, or add new endpoint support unless
the user explicitly asks to revive Bidask support.

## Release / Publishing

`bidask_api_client` is intentionally excluded from the release-plz publish cycle
with `publish = false` in `Cargo.toml`. Do not re-enable publishing or prepare
crates.io release metadata for this crate unless the user explicitly asks to
revive Bidask support.

## Public API Boundary

Treat these as public contracts:

- `BidaskApiClient`
- `DEFAULT_API_URL`
- `api::ApiClient`
- `Request`
- `Response`
- public response/type structs
- `unwrap_response!`

Public request/response types are `#[non_exhaustive]`; downstream matches need
wildcard arms, and examples should avoid struct literal construction for public
models. Public POD structs still support `Default::default().with_<field>(...)`
for tests and migration code, but do not document or recommend this crate for
final-app integration unless the user explicitly revives Bidask support.

Preserve pagination behavior unless the user explicitly asks for page-level
control. If adding endpoints, keep pagination decisions explicit in the request
or response type.

## Live API Notes

The live API test is intentionally commented out while this crate remains
unsupported. Do not re-enable Bidask live coverage unless the user explicitly
asks to revive Bidask support.

## Validation

```bash
cargo test -p bidask_api_client --tests
cargo +nightly fmt
cargo clippy -p bidask_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p bidask_api_client --no-deps
cargo package --list -p bidask_api_client
```
