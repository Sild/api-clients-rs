use anyhow::Result;
use api_clients_core::Executor;
use dedust_api_client::api_client::{DedustApiClient, DEFAULT_API_V4_URL};
use dedust_api_client::unwrap_v4_response;
use dedust_api_client::v4::{PoolsParams, V4Request};
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

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env()?;
    let params = PoolsParams::new().with_limit(1).with_sort_by("tvl".to_owned());
    let response = client.v4.pools(&params).await?;

    assert!(response.total_count > 0);
    assert_eq!(response.pool_rows.len(), 1);
    assert!(!response.pool_rows[0].pools.is_empty());
    Ok(())
}

#[tokio::test]
#[ignore = "all_pools performs multiple sequential requests against the live DeDust service"]
async fn test_all_pools() -> Result<()> {
    let client = init_env()?;
    let all_params =
        PoolsParams::new().with_offset(u32::MAX).with_limit(10).with_filter_by_type(vec!["stable".to_owned()]);
    let all_response = client.v4.all_pools(&all_params).await?;
    assert_eq!(u32::try_from(all_response.pool_rows.len())?, all_response.total_count);
    Ok(())
}
