# stonks_api_client

Thin typed wrapper for the public [Stonks](https://app.stonks.cash) endpoints.

Use this crate to load public-token tax metadata or discover Stonks Virtual
Pool addresses. The crate preserves the raw tax percentages returned by
Stonks; fee conversion, pool hydration, filtering, and routing belong in the
application layer.

## Usage

```toml
[dependencies]
stonks_api_client = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The minimum supported Rust version (MSRV) is 1.88.

Run requests inside an async Tokio runtime. Pass [`PoolsParams`] directly to
the API client for one page, or use [`Request::AllPools`] to load every page.

```rust,no_run
use stonks_api_client::api::{Request, Response};
use stonks_api_client::api_client::StonksApiClient;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = StonksApiClient::builder().build()?;
let response = client.api.exec(Request::AllPools).await?;

match response {
    Response::AllPools(addresses) => println!("Virtual Pools: {}", addresses.len()),
    _ => println!("unexpected Stonks response variant"),
}
# Ok(())
# }
```

[`PoolsParams`]: api::PoolsParams
[`Request::AllPools`]: api::Request::AllPools

## Supported Endpoints

| Method                                                    | Supported |
|-----------------------------------------------------------|-----------|
| `/api/deployments/public-tokens`                          | ✅        |
| `/api/virtual-deployments/non-bonded-tokens?page=&size=` | ✅        |

Public request and response types are marked `#[non_exhaustive]` for semver
headroom. Build public POD structs with `Default::default().with_<field>(...)`
or constructors, pass request parameters directly where `Into<Request>` is
implemented, and include wildcard arms when matching response enums.

`Request::AllPools` starts at page zero, requests 100 addresses per page, and
returns only after every page succeeds. Live API results can change as tokens
bond and new deployments are created.
