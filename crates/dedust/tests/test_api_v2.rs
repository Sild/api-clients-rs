use anyhow::Result;
use api_clients_core::Executor;
use dedust_api_client::api_client::DedustApiClient;
use dedust_api_client::api_client::DEFAULT_API_V2_URL;
use dedust_api_client::unwrap_response;
use dedust_api_client::v2::{RoutingPlanParams, V2Request};
use std::sync::Arc;
use std::time::Duration;

fn init_env() -> Result<DedustApiClient> {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    let executor = Executor::builder(DEFAULT_API_V2_URL).with_timeout(Duration::from_secs(60)).build()?;
    Ok(DedustApiClient::builder().with_executor(Arc::new(executor)).build()?)
}

#[tokio::test]
async fn test_assets() -> Result<()> {
    let client = init_env()?;
    let request = V2Request::Assets;
    let response = unwrap_response!(Assets, client.v2.exec(&request).await?)?;
    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env()?;
    let request = V2Request::Pools;
    let response = unwrap_response!(Pools, client.v2.exec(&request).await?)?;
    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_pools_lite() -> Result<()> {
    let client = init_env()?;
    let request = V2Request::PoolsLite;
    let response = unwrap_response!(PoolsLite, client.v2.exec(&request).await?)?;
    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_pool_trades() -> Result<()> {
    let client = init_env()?;
    let request = V2Request::PoolTrades("EQAADLqcF3lNb1O_GQLowwky8vvUGXuzPRNGvKBwBxjsHR7s".to_string());
    let response = unwrap_response!(PoolTrades, client.v2.exec(&request).await?)?;
    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_routing_plan() -> Result<()> {
    let client = init_env()?;
    let usdt_addr = "0:b113a994b5024a16719f69139328eb759596c38a25f59028b146fecdc3621dfe";
    let ton_addr = "0:0000000000000000000000000000000000000000000000000000000000000000";
    let params = RoutingPlanParams::new(usdt_addr, ton_addr, &10.to_string());
    let response = unwrap_response!(RoutingPlan, client.v2.exec(params).await?)?;
    assert!(!response.is_empty());
    assert!(!response[0].is_empty());
    Ok(())
}
