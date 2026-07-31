use anyhow::Result;
use api_clients_core::Executor;
use dedust_api_client::api_client::{DedustApiClient, DEFAULT_API_V4_URL};
use dedust_api_client::unwrap_v4_response;
use dedust_api_client::v4::V4Request;
use std::sync::Arc;
use std::time::Duration;

fn init_env() -> Result<DedustApiClient> {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    let executor = Executor::builder(DEFAULT_API_V4_URL).with_timeout(Duration::from_secs(60)).build()?;
    Ok(DedustApiClient::builder().with_v4_executor(Arc::new(executor)).build()?)
}

#[tokio::test]
async fn test_all_classic_pools() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_v4_response!(AllClassicPools, client.v4.exec(V4Request::AllClassicPools).await?)?;

    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_all_stable_pools() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_v4_response!(AllStablePools, client.v4.exec(V4Request::AllStablePools).await?)?;

    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_all_cpmm_pools() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_v4_response!(AllCpmmPools, client.v4.exec(V4Request::AllCpmmPools).await?)?;

    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_all_uranus_pools() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_v4_response!(AllUranusPools, client.v4.exec(V4Request::AllUranusPools).await?)?;

    assert!(!response.is_empty());
    Ok(())
}
