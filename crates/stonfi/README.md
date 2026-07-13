# stonfi_api_client

Thin typed wrapper for the [STON.fi API v1 and public export endpoints](https://api.ston.fi/swagger-ui).

Use this crate when an application needs typed access to STON.fi assets, pools,
farms, routers, swap/liquidity simulation, wallet views, stats, transactions, or
public export feeds. The crate does not execute swaps or manage transaction
submission; application code owns wallet signing, retries around business
workflows, persistence, and fallback behavior.

## Usage

```toml
[dependencies]
stonfi_api_client = "0.8"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Run requests inside an async Tokio runtime. Pass request parameter structs
directly where `Into<V1Request>` or `Into<ExportRequest>` is implemented, and
match response enums with a wildcard arm.

```rust,no_run
use stonfi_api_client::api_client::StonfiApiClient;
use stonfi_api_client::v1::{PoolsParams, V1Response};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = StonfiApiClient::builder().build()?;
let response = client.v1.exec(PoolsParams::new()).await?;

match response {
    V1Response::Pools(pools) => {
        println!("pools: {}", pools.pool_list.len());
    }
    _ => {
        println!("unexpected STON.fi response variant");
    }
}
# Ok(())
# }
```

Use `ExportApiClient` for public export feeds:

```rust,no_run
use stonfi_api_client::api_client::StonfiApiClient;
use stonfi_api_client::export::{ExportRequest, ExportResponse};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = StonfiApiClient::builder().build()?;
let response = client.export.exec(ExportRequest::Cmc).await?;

match response {
    ExportResponse::Cmc(pools) => println!("CMC pools: {}", pools.len()),
    _ => println!("unexpected STON.fi export response variant"),
}
# Ok(())
# }
```

## Supported Endpoints

| Group | Method                                        | Supported |
|-------|-----------------------------------------------|----------|
| AMM   | /v1/assets                                    | ✅        |
| AMM   | /v1/assets/query                              | ✅        |
| AMM   | /v1/assets/search                             | ✅        |
| AMM   | /v1/assets/{addr_str}                         | ✅        |
| AMM   | /v1/farms                                     | ✅        |
| AMM   | /v1/farms/by_pool/{pool_addr_str}             | ✅        |
| AMM   | /v1/farms/{addr_str}                          | ✅        |
| AMM   | /v1/jetton/{addr_str}/address                 | ✅        |
| AMM   | /v1/liquidity_provision/simulate              | ✅        |
| AMM   | /v1/markets                                   | ✅        |
| AMM   | /v1/pools/query                               | ✅        |
| AMM   | /v1/pools                                     | ✅        |
| AMM   | /v1/pools/by_market/{asset_0_addr_str}/{asset_1_addr_str} | ✅        |
| AMM   | /v1/pools/{addr_str}                          | ✅        |
| AMM   | /v1/reverse_swap/simulate                     | ✅        |
| AMM   | /v1/routers                                   | ✅        |
| AMM   | /v1/routers/{addr_str}                        | ✅        |
| AMM   | /v1/stats/dex                                 | ✅        |
| AMM   | /v1/stats/fee_accruals                        | ✅        |
| AMM   | /v1/stats/fee_withdrawals                     | ✅        |
| AMM   | /v1/stats/fees                                | ✅        |
| AMM   | /v1/stats/operations                          | ✅        |
| AMM   | /v1/stats/pool                                | ✅        |
| AMM   | /v1/stats/staking                             | ✅        |
| AMM   | /v1/swap/simulate                             | ✅        |
| AMM   | /v1/swap/status                               | ✅        |
| AMM   | /v1/transactions/query                        | ✅        |
| AMM   | /v1/transactions/{hash}/action_tree           | ✅        |
| AMM   | /v1/wallets/{addr_str}/assets                 | ✅        |
| AMM   | /v1/wallets/{addr_str}/assets/{asset_str}     | ✅        |
| AMM   | /v1/wallets/{addr_str}/farms                  | ✅        |
| AMM   | /v1/wallets/{addr_str}/farms/{farm_str}       | ✅        |
| AMM   | /v1/wallets/{addr_str}/fee_vaults             | ✅        |
| AMM   | /v1/wallets/{addr_str}/operations             | ✅        |
| AMM   | /v1/wallets/{addr_str}/pools                  | ✅        |
| AMM   | /v1/wallets/{addr_str}/pools/{pool_str}       | ✅        |
| AMM   | /v1/wallets/{addr_str}/stakes                 | ✅        |
| AMM   | /v1/wallets/{addr_str}/transactions/last      | ✅        |
| Export | /export/cmc/v1                               | ✅        |
| Export | /export/dexscreener/v1/asset/{address}       | ✅        |
| Export | /export/dexscreener/v1/events                | ✅        |
| Export | /export/dexscreener/v1/latest-block          | ✅        |
| Export | /export/dexscreener/v1/pair/{address}        | ✅        |

Public request and response types are marked `#[non_exhaustive]` for semver
headroom. Build public POD structs with `Default::default().with_<field>(...)`
or request parameter `new()` constructors, pass request parameters directly to
`V1ApiClient::exec` or `ExportApiClient::exec` where `Into<V1Request>` or
`Into<ExportRequest>` is implemented, and include a wildcard arm when matching
response enums.

Live API tests hit STON.fi directly. Asset metadata, pool counts, prices,
transaction state, and optional transaction fields can drift with upstream data.
