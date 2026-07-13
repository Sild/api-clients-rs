# AGENTS.md

## Scope

This crate is `stonfi_api_client`, a Rust library crate that wraps STON.fi REST
API v1 and public export endpoints.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a thin typed client:

- `StonfiApiClient::builder().build()?`
- `client.v1.exec(&V1Request::...)`
- `client.export.exec(&ExportRequest::...)`
- request params in `v1/request.rs`
- response enums and response wrappers in `v1/response.rs` and models/actions in
  `v1/types.rs`
- export dispatch in `export.rs`, requests in `export/request.rs`, responses in
  `export/response.rs`, and export models in `export/types.rs`

Do not add application-level swap orchestration or transaction execution logic
here. This crate only wraps STON.fi API responses.

## Public API Boundary

Treat these as public contracts:

- `StonfiApiClient`
- `DEFAULT_API_URL`
- `DEFAULT_API_V1_URL`
- `V1ApiClient`
- `V1Request`
- `ExportApiClient`
- `ExportRequest`
- all public request parameter structs
- `V1Response`, `ExportResponse`, and public response/action/type structs
- `unwrap_response!`

Request parameter and response/model POD structs are `#[non_exhaustive]`; use
`Default::default().with_<field>(...)` or request parameter `new()`
constructors instead of struct literals in downstream examples and integration
tests. Pass request parameters directly to `V1ApiClient::exec` where
`Into<V1Request>` is implemented. Public enums are `#[non_exhaustive]`;
downstream matches need wildcard arms.

When adding an endpoint, update the README support matrix and add coverage that
proves the request path, method, params, and response parsing.

## Live API Notes

Tests in `tests/test_api.rs` hit the live STON.fi API. STON.fi asset metadata
and historical transaction fixtures can drift. Verify the live response before
changing client code.

Known recent contract observations:

- Native TON metadata has returned `display_name = "Gram"` and
  `symbol = "GRAM"`.
- Old transaction query fixtures may return no `tx_id`; avoid unwrapping
  fixture-dependent optional fields.

## Downstream Integration Example

```rust
use stonfi_api_client::api_client::StonfiApiClient;
use stonfi_api_client::v1::{PoolsParams, V1Request, V1Response};

# async fn example() -> anyhow::Result<()> {
let client = StonfiApiClient::builder().build()?;
let response = client.v1.exec(PoolsParams::default()).await?;

match response {
    V1Response::Pools(pools) => {
        println!("pools: {}", pools.pool_list.len());
    }
    other => {
        anyhow::bail!("unexpected STON.fi response: {other:?}");
    }
}
# Ok(())
# }
```

Final applications should treat values such as asset names, prices, transaction
status, and optional transaction IDs as live external data.

## Validation

```bash
cargo test -p stonfi --tests
cargo +nightly fmt
cargo clippy -p stonfi --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p stonfi --no-deps
cargo package --list -p stonfi
```
