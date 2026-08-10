# AGENTS.md

## Scope

This crate is `stonks_api_client`, a public Rust library that wraps Stonks
public-token metadata, batch jetton metadata, and Virtual Pool address-discovery
endpoints.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a thin typed client:

- `StonksApiClient::builder().build()?`
- `client.api.exec(Request::PublicTokens)`
- `client.api.exec(VirtualPoolAddressesParams::new(page, size))`
- `client.api.exec(Request::AllVirtualPoolAddresses)`
- `client.api.exec(JettonMetadataBatchRequest::new(ids))`
- request parameters in `api/request.rs`
- response enums and wire models in `api/response.rs` and `api/types.rs`

Keep pool hydration, tax conversion, filtering, routing, and application retry
policy outside this crate.

## Public API Boundary

Treat these as public contracts:

- `StonksApiClient`
- `DEFAULT_API_URL`
- `api::ApiClient`
- `Request`, `VirtualPoolAddressesParams`, and `JettonMetadataBatchRequest`
- `Response`, `PublicToken`, and `JettonMetadata`
- `unwrap_response!`

Request parameter and response/model POD structs are `#[non_exhaustive]`; use
constructors or `Default::default().with_<field>(...)` rather than struct
literals in downstream code. Public enums are also `#[non_exhaustive]`, so
downstream matches need wildcard arms.

The API returns `buyTax` and `sellTax` as raw percentage integers. Do not
convert them to basis points or apply an application-specific fee. Pool
discovery returns raw TON address strings rather than hydrated pool objects.

`Request::AllVirtualPoolAddresses` owns zero-based pagination with a fixed page
size of 100. It preserves response ordering and duplicates, stops after a short
page, returns no partial result if a request fails, and must remain sequential
unless the public behavior is deliberately redesigned.

`VirtualPoolAddressesParams` uses fixed-width `u32` values for both `page` and
`size`; do not replace wire pagination with target-width integer types.

## Downstream Integration Example

```rust
use stonks_api_client::api::{Request, Response};
use stonks_api_client::api_client::StonksApiClient;

# async fn example() -> anyhow::Result<()> {
let client = StonksApiClient::builder().build()?;
let response = client.api.exec(Request::PublicTokens).await?;

match response {
    Response::PublicTokens(tokens) => println!("public tokens: {}", tokens.len()),
    other => anyhow::bail!("unexpected Stonks response: {other:?}"),
}
# Ok(())
# }
```

## Live API Notes

Tests in `tests/test_api.rs` call Stonks directly. Avoid assertions on volatile
address ordering, total counts, symbols, or tax values. Validate endpoint
routing, response shape, and required field parsing.

## Changing The API

Keep endpoint paths in `api.rs` and wire names in the request/response modules.
When the public API or supported endpoints change, update this file, the crate
README and rustdoc, integration tests, the root service table, and package
surface checks together.

## Validation

```bash
cargo test -p stonks_api_client --tests
cargo test -p stonks_api_client --doc
cargo +nightly fmt
cargo clippy -p stonks_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p stonks_api_client --no-deps
cargo +1.88.0 check -p stonks_api_client --all-targets --all-features
cargo package --list -p stonks_api_client
cargo publish --dry-run -p stonks_api_client
```

Version bumps and generated release changelog entries are owned by release-plz.
