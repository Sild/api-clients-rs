# api_clients_rs

Thin Rust API-client wrappers for TON ecosystem services. Each crate stays close
to its upstream API contract and exposes typed request/response models without
adding application-specific swap, routing, persistence, or fallback logic.

MRs are welcome.

| Service               | Client                                                   | Status      | Capabilities |
|-----------------------|----------------------------------------------------------|-------------|--------------|
| https://ston.fi       | [stonfi_api_client](crates/stonfi)                       | Supported   | STON.fi API v1 assets, pools, farms, routers, swap/liquidity simulation, wallet views, stats, transactions, and public export feeds. |
| https://app.stonks.cash | [stonks_api_client](crates/stonks)                     | Supported   | Stonks public-token tax metadata and paginated Virtual Pool address discovery. |
| https://dedust.io     | [dedust_api_client](crates/dedust)                       | Supported   | DeDust asset metadata, v4 Classic/Stable/CPMM v2/Uranus pool registries, and legacy v2 operations. |
| https://app.tonco.io/ | [tonco_api_client](crates/tonco_api_client)              | Supported   | Low-level Tonco Indexer GraphQL execution with caller-owned query/schema files and generated types. |
| https://swap.coffee   | [swap_coffee_api_client](crates/swap_coffee)             | Supported   | Swap Coffee API v1 tokens and pools. |
| Bidask                | [bidask_api_client](crates/bidask)                       | Unsupported | Legacy source only; not recommended for application integration and not published. |


If you're interested in some particular endpoint which is not implemented yet, just raise an issue and I'll add it.

## Usage

Add the crate for the service you need:

```toml
[dependencies]
stonfi_api_client = "0.9"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The workspace minimum supported Rust version (MSRV) is 1.88.

Then build a client and execute the typed request in an async Tokio runtime:

```rust
use stonfi_api_client::api_client::StonfiApiClient;
use stonfi_api_client::v1::{PoolsParams, V1Response};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = StonfiApiClient::builder().build()?;
let response = client.v1.exec(PoolsParams::default()).await?;

match response {
    V1Response::Pools(pools) => println!("pools: {}", pools.pool_list.len()),
    _ => println!("unexpected response variant"),
}
# Ok(())
# }
```

Public request and response types are marked `#[non_exhaustive]` where the
workspace needs room to add fields or enum variants in future minor releases.
Build public POD structs with `Default::default().with_<field>(...)` or request
parameter constructors instead of struct literals, pass request parameters
directly to clients where `Into<Request>` is implemented, and keep wildcard
arms when matching public enums.

All service clients use `api_clients_core::Executor` underneath. The default
executor retries transient failures, uses a 10-second timeout, and applies a
smooth 10 RPS client-side rate limit. Applications that need a different
transport policy can build an `Executor` directly and inject it through the
service client builder.

Integration tests call the real upstream services and are intentionally not run
in CI/CD. Run live checks manually before pushing to a remote:

```bash
cargo test --workspace --tests
```
