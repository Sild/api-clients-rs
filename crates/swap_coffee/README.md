# swap_coffee_api_client

Thin typed wrapper for the [Swap Coffee API v1](https://backend.swap.coffee/v1).

Use this crate when an application needs typed access to Swap Coffee token or
pool data. The crate does not make routing decisions, execute swaps, or enforce
trade-safety policy; keep those decisions in the application layer.

## Usage

```toml
[dependencies]
swap_coffee_api_client = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Run requests inside an async Tokio runtime. Pass request parameter structs
directly where `Into<V1Request>` is implemented, and match response enums with a
wildcard arm.

```rust,no_run
use swap_coffee_api_client::api_client::SwapCoffeeApiClient;
use swap_coffee_api_client::v1::{Dexes, V1Response};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = SwapCoffeeApiClient::builder().build()?;
let response = client.v1.exec(Dexes::new("coffee")).await?;

match response {
    V1Response::Pools(pages) => println!("pool pages: {}", pages.len()),
    _ => println!("unexpected Swap Coffee response variant"),
}
# Ok(())
# }
```

## Supported Endpoints

| Method                                   | Supported |
|------------------------------------------|-----------|
| /v1/pools                                | ✅        |
| /v1/tokens                               | ✅        |

Public request and response types are marked `#[non_exhaustive]` for semver
headroom. Build public POD structs with `Default::default().with_<field>(...)`
or request parameter constructors, pass request parameters directly to
`client.v1.exec` where `Into<V1Request>` is implemented, and include a wildcard
arm when matching response enums.

Live API tests hit Swap Coffee directly. Pool counts, token ordering, metadata,
and liquidity fields can drift with upstream state.
