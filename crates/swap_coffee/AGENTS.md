# AGENTS.md

## Scope

This crate is `swap_coffee_api_client`, a Rust library crate that wraps Swap
Coffee REST API v1.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a thin typed client:

- `SwapCoffeeApiClient::builder().build()?`
- `client.v1.exec(&V1Request::...)`
- request params in `v1/request.rs`
- response enums and models in `v1/response.rs` and `v1/types.rs`

Do not add routing decisions or swap execution flows here. Keep this crate to
API request/response wrapping.

## Public API Boundary

Treat these as public contracts:

- `SwapCoffeeApiClient`
- `DEFAULT_API_V1_URL`
- `V1ApiClient`
- `V1Request`
- `Dexes`
- `V1Response` and public response/type structs
- `unwrap_response!`

Request parameter and response/model POD structs are `#[non_exhaustive]`; use
`Default::default().with_<field>(...)` or request parameter `new()` methods
instead of struct literals in downstream examples and integration tests. Pass
request parameters directly to `client.v1.exec` where `Into<V1Request>` is
implemented. Public enums are `#[non_exhaustive]`; downstream matches need
wildcard arms.

`Dexes` is serialized into query parameters for `/pools`; preserve that shape
unless the upstream API changes.

## Live API Notes

Tests in `tests/test_api_v1.rs` hit the live Swap Coffee API. Avoid brittle
assertions on returned pool counts, token ordering, or dynamic metadata.

## Downstream Integration Example

```rust
use swap_coffee_api_client::api_client::SwapCoffeeApiClient;
use swap_coffee_api_client::v1::{Dexes, V1Request, V1Response};

# async fn example() -> anyhow::Result<()> {
let client = SwapCoffeeApiClient::builder().build()?;
let response = client.v1.exec(Dexes::new("stonfi_v2")).await?;

match response {
    V1Response::Pools(pools) => println!("pool pages: {}", pools.len()),
    other => anyhow::bail!("unexpected Swap Coffee response: {other:?}"),
}
# Ok(())
# }
```

Final applications should keep routing preferences, slippage, and trade safety
checks outside this crate.

## Validation

```bash
cargo test -p swap_coffee_api_client --tests
cargo +nightly fmt
cargo clippy -p swap_coffee_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p swap_coffee_api_client --no-deps
cargo package --list -p swap_coffee_api_client
```
