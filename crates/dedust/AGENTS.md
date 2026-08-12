# AGENTS.md

## Scope

This crate is `dedust_api_client`, a public Rust library crate that wraps the
DeDust asset registry, API v4 pool discovery endpoints, and legacy REST API v2.

Use the repository root `AGENTS.md` first, then this file. Use the
`rust-library-review` skill for public API, docs, package, or agent-guidance
changes.

## Crate Purpose

The crate exposes a thin typed client:

- `DedustApiClient::builder().build()?`
- `client.assets.exec(&AssetsRequest::...)`
- `client.v2.exec(&V2Request::...)`
- `client.v4.exec(&V4Request::...)`
- raw request, response, and model modules under `assets/`, `v2/`, and `v4/`

Keep DeDust-specific address formatting and endpoint mapping in this crate.
Do not join asset metadata into pool registries or derive dynamic pool state.

## Public API Boundary

Treat these as public contracts:

- `DedustApiClient`
- `DEFAULT_ASSETS_URL`
- `DEFAULT_API_V2_URL`
- `DEFAULT_API_V4_URL`
- `AssetsApiClient`, `AssetsRequest`, `AssetsResponse`, and asset wire models
- `V2ApiClient`
- `V2Request`
- `RoutingPlanParams`
- `V2Response` and public response/type structs
- `V4ApiClient`, `V4Request`, `V4Response`, `PoolsParams`,
  `V4ApiClient::pools`, `V4ApiClient::all_pools`, and v4 pool wire models
- `unwrap_response!`
- `unwrap_assets_response!`
- `unwrap_v4_response!`

Request parameter and response/model POD structs are `#[non_exhaustive]`; use
`Default::default().with_<field>(...)` or request parameter constructors instead
of struct literals in downstream examples and integration tests. Public enums
are `#[non_exhaustive]`; downstream matches need wildcard arms.

The existing `with_api_url` and `with_executor` builder setters configure v2.
Use `with_assets_url`/`with_assets_executor` and
`with_v4_url`/`with_v4_executor` for the other origins.

`RoutingPlanParams::new` maps the zero TON address to `native` and all other
addresses to `jetton:<address>`. Do not change that mapping without validating
DeDust API expectations and updating examples.

## Live API Notes

Tests in `tests/test_assets.rs`, `tests/test_api_v2.rs`, and
`tests/test_api_v4.rs` hit the live DeDust services. Prefer assertions that
prove endpoint support and response parsing without relying on volatile asset
or pool counts, routing amounts, or ordering.
`test_all_pools` is ignored by default because it makes multiple sequential live
requests; run it explicitly when changing automatic pagination.

The asset registry uses friendly TON addresses and absolute image URLs. The v4
pool endpoints use raw `workchain:hex_hash` addresses and asset identifiers such
as `native` and `jetton:0:<hash>`. The `get_pools_all*` registries provide static
discovery/configuration records. The frontend-oriented `POST /get_pools`
screener provides grouped pools with TVL, volume, APR, reserves, fees, rewards,
and embedded asset metadata; its page limit and the `PoolsParams` default are
currently 100. `all_pools` starts at offset zero, loads pages sequentially, and
returns no partial response on failure. Keep decimal and raw integer values as
strings. Keep legacy v2 asset and pool operations
available for backward compatibility, but do not recommend them for new
registry integrations.

Treat `POST /get_pools` as an observed DeDust web-application interface rather
than a documented stable developer endpoint. Confirm its live wire contract
before changing its request or response types.

## Downstream Integration Example

```rust
use dedust_api_client::api_client::DedustApiClient;
use dedust_api_client::assets::{AssetsRequest, AssetsResponse};
use dedust_api_client::v4::PoolsParams;

# async fn example() -> anyhow::Result<()> {
let client = DedustApiClient::builder().build()?;
let assets_response = client.assets.exec(AssetsRequest::List).await?;
let params = PoolsParams::new().with_limit(100);
let pools_response = client.v4.all_pools(&params).await?;

match assets_response {
    AssetsResponse::List(assets) => println!("assets: {}", assets.len()),
    other => anyhow::bail!("unexpected DeDust asset response: {other:?}"),
}
println!("screened pool rows: {}", pools_response.total_count);
# Ok(())
# }
```

Final applications should keep address conversion, pool hydration, amount and
slippage interpretation, persistence, and fallback behavior in their own
domain layer.

## Validation

```bash
cargo test -p dedust_api_client --tests
cargo test -p dedust_api_client --test test_api_v4 test_all_pools -- --ignored --exact
cargo +nightly fmt
cargo clippy -p dedust_api_client --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -p dedust_api_client --no-deps
cargo package --list -p dedust_api_client
```
