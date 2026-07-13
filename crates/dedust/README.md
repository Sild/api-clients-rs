# dedust_api_client

Thin typed wrapper for the [DeDust API v2](https://api.dedust.io/).

Use this crate when an application needs typed access to DeDust assets, pools,
pool trades, or routing plans. The crate does not choose routes, calculate
slippage, execute swaps, or normalize DeDust data into a shared DEX domain model.

## Usage

```toml
[dependencies]
dedust_api_client = "0.6"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Run requests inside an async Tokio runtime. Pass request parameter structs
directly where `Into<V2Request>` is implemented, and match response enums with a
wildcard arm.

```rust,no_run
use dedust_api_client::api_client::DedustApiClient;
use dedust_api_client::v2::{RoutingPlanParams, V2Response};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = DedustApiClient::builder().build()?;
let params = RoutingPlanParams::new(
    "0:0000000000000000000000000000000000000000000000000000000000000000",
    "0:0000000000000000000000000000000000000000000000000000000000000000",
    "1000000000",
);
let response = client.v2.exec(params).await?;

match response {
    V2Response::RoutingPlan(routes) => println!("route groups: {}", routes.len()),
    _ => println!("unexpected DeDust response variant"),
}
# Ok(())
# }
```

`RoutingPlanParams::new` maps the zero TON address to `native` and all other
addresses to `jetton:<address>`.

## Supported Endpoints

| Method                                   | Supported |
|------------------------------------------|-----------|
| /v2/accounts/{address}/assets            |           |
| /v2/accounts/{address}/trades            |           |
| /v2/assets                               | ✅         |
| /v2/assets/{symbol}                      |           |
| /v2/coinmarketcap/markets                |           |
| /v2/dns/{domain}                         |           |
| /v2/gcko/pairs                           |           |
| /v2/gcko/tickers                         |           |
| /v2/gcko/trades                          |           |
| /v2/jettons/{address}/circulating-supply |           |
| /v2/jettons/{address}/metadata           |           |
| /v2/jettons/{address}/top-buys           |           |
| /v2/jettons/{address}/top-traders        |           |
| /v2/jettons/{address}/total-supply       |           |
| /v2/pools                                | ✅         |
| /v2/pools-lite                           | ✅         |
| /v2/pools/{address}/metadata             |           |
| /v2/pools/{address}/trades               | ✅         |
| /v2/prices                               |           |
| /v2/routing/plan                         | ✅         |

Public request and response types are marked `#[non_exhaustive]` for semver
headroom. Build public POD structs with `Default::default().with_<field>(...)`
or request parameter constructors, pass request parameters directly to
`client.v2.exec` where `Into<V2Request>` is implemented, and include a wildcard
arm when matching response enums.

Live API tests hit DeDust directly. Pool counts, routing amounts, and ordering
can drift with upstream state.
