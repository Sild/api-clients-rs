use anyhow::{Context, Result};
use stonks_api_client::api::{PoolsParams, Request};
use stonks_api_client::api_client::StonksApiClient;
use stonks_api_client::unwrap_response;

fn init_client() -> Result<StonksApiClient> { Ok(StonksApiClient::builder().build()?) }

#[tokio::test]
async fn test_assets() -> Result<()> {
    let client = init_client()?;
    let assets = unwrap_response!(Assets, client.api.exec(Request::Assets).await?)?;
    let asset = assets.first().context("Stonks returned no public tokens")?;

    assert!(!asset.symbol.is_empty());
    assert!(!asset.address.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_pool_page() -> Result<()> {
    let client = init_client()?;
    let pools = unwrap_response!(Pools, client.api.exec(PoolsParams::new(0, 1)).await?)?;
    let address = pools.first().context("Stonks returned no Virtual Pool addresses")?;

    assert!(pools.len() <= 1);
    assert!(!address.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_all_pools() -> Result<()> {
    let client = init_client()?;
    let pools = unwrap_response!(AllPools, client.api.exec(Request::AllPools).await?)?;

    assert!(!pools.is_empty());
    assert!(pools.iter().all(|address| !address.is_empty()));
    Ok(())
}
