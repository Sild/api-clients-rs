# dedust_api_client

Thin typed wrapper for the [DeDust](https://dedust.io/) asset registry, API v4
pool registries, and legacy API v2.

Use this crate when an application needs raw typed access to DeDust asset
metadata, pool discovery/configuration, pool trades, or routing plans. The crate
does not join asset metadata into pools, load on-chain pool state, choose routes,
calculate slippage, execute swaps, or normalize DeDust data into a shared DEX
domain model.

## Usage

```toml
[dependencies]
dedust_api_client = "0.7"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The minimum supported Rust version (MSRV) is 1.88.

Run requests inside an async Tokio runtime. Match response enums with a wildcard
arm because they are non-exhaustive.

```rust,no_run
use dedust_api_client::api_client::DedustApiClient;
use dedust_api_client::assets::{AssetsRequest, AssetsResponse};
use dedust_api_client::v4::{V4Request, V4Response};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = DedustApiClient::builder().build()?;

let assets_response = client.assets.exec(AssetsRequest::List).await?;
match assets_response {
    AssetsResponse::List(assets) => println!("assets: {}", assets.len()),
    _ => println!("unexpected DeDust asset response variant"),
}

let pools_response = client.v4.exec(V4Request::AllCpmmPools).await?;
match pools_response {
    V4Response::AllCpmmPools(pools) => println!("CPMM v2 pools: {}", pools.len()),
    _ => println!("unexpected DeDust v4 response variant"),
}
# Ok(())
# }
```

The asset registry returns friendly TON addresses and absolute image URLs. The
v4 pool registries return raw `workchain:hex_hash` addresses and asset strings
such as `native` and `jetton:0:<hash>`. Applications own any address conversion,
metadata joins, and dynamic pool-state hydration.

## Asset Registry

| Method | Endpoint   | Request               | Response              |
|--------|------------|-----------------------|-----------------------|
| GET    | /list.json | `AssetsRequest::List` | `AssetsResponse::List` |

The default asset-registry origin is `https://assets.dedust.io`.

## API v4 Pool Registries

| Method | Endpoint                  | Request                            | Response                           |
|--------|---------------------------|------------------------------------|------------------------------------|
| GET    | /get_pools_allclassic     | `V4Request::AllClassicPools`       | `V4Response::AllClassicPools`      |
| GET    | /get_pools_allstable      | `V4Request::AllStablePools`        | `V4Response::AllStablePools`       |
| GET    | /get_pools_allcpmm        | `V4Request::AllCpmmPools`          | `V4Response::AllCpmmPools`         |
| GET    | /get_pools_alluranus      | `V4Request::AllUranusPools`        | `V4Response::AllUranusPools`       |

The default v4 origin is `https://mainnet.api.dedust.io/v4/api`. These endpoints
return discovery and configuration records. They do not include v2 fields such
as reserves, liquidity-token supply, logical time, last price, volume, or
accumulated fees. Uranus records identify launchpad tokens and fundraising
configuration rather than ordinary two-asset pools.

## Legacy API v2

The following operations remain available through `client.v2` for backward
compatibility. The v2 asset and pool-list endpoints are considered legacy and
should not be selected for new registry integrations.

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

`RoutingPlanParams::new` maps the zero TON address to `native` and all other
addresses to `jetton:<address>`.

Public request and response types are marked `#[non_exhaustive]` for SemVer
headroom. Build public POD structs with `Default::default().with_<field>(...)`
or request parameter constructors, pass request parameters directly where
`Into<Request>` is implemented, and include a wildcard arm when matching public
enums.

The existing `with_api_url` and `with_executor` builder setters configure v2.
Use `with_assets_url`/`with_assets_executor` and
`with_v4_url`/`with_v4_executor` to override the other origins independently.

Live API tests hit DeDust directly. Asset and pool counts, metadata, routing
amounts, and ordering can drift with upstream state.
