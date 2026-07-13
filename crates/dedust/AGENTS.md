# AGENTS.md

## Scope

This crate is `dedust_api_client`, a Rust library crate that wraps DeDust REST
API v2.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a thin typed client:

- `DedustApiClient::builder().build()?`
- `client.v2.exec(&V2Request::...)`
- request params in `v2/request.rs`
- response enums and models in `v2/response.rs` and `v2/types.rs`

Keep DeDust-specific address formatting and endpoint mapping in this crate.

## Public API Boundary

Treat these as public contracts:

- `DedustApiClient`
- `DEFAULT_API_V2_URL`
- `V2ApiClient`
- `V2Request`
- `RoutingPlanParams`
- `V2Response` and public response/type structs
- `unwrap_response!`

Request parameter and response/model POD structs are `#[non_exhaustive]`; use
`Default::default().with_<field>(...)` or request parameter constructors instead
of struct literals in downstream examples and integration tests. Pass request
parameters directly to `client.v2.exec` where `Into<V2Request>` is implemented.
Public enums are `#[non_exhaustive]`; downstream matches need wildcard arms.

`RoutingPlanParams::new` maps the zero TON address to `native` and all other
addresses to `jetton:<address>`. Do not change that mapping without validating
DeDust API expectations and updating examples.

## Live API Notes

Tests in `tests/test_api_v2.rs` hit the live DeDust API. Prefer assertions that
prove endpoint support and response parsing without relying on volatile pool
counts, routing amounts, or ordering.

## Downstream Integration Example

```rust
use dedust_api_client::api_client::DedustApiClient;
use dedust_api_client::v2::{RoutingPlanParams, V2Request, V2Response};

# async fn example() -> anyhow::Result<()> {
let client = DedustApiClient::builder().build()?;
let params = RoutingPlanParams::new(
    "0:0000000000000000000000000000000000000000000000000000000000000000",
    "0:0000000000000000000000000000000000000000000000000000000000000000",
    "1000000000",
);
let response = client.v2.exec(params).await?;

match response {
    V2Response::RoutingPlan(routes) => println!("routes: {}", routes.len()),
    other => anyhow::bail!("unexpected DeDust response: {other:?}"),
}
# Ok(())
# }
```

Final applications should keep address, amount, and slippage interpretation in
their own domain layer.

## Validation

```bash
cargo test -p dedust_api_client --tests
cargo +nightly fmt
cargo clippy -p dedust_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p dedust_api_client --no-deps
cargo package --list -p dedust_api_client
```
