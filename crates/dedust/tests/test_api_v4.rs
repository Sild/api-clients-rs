use anyhow::Result;
use api_clients_core::Executor;
use dedust_api_client::api_client::{DedustApiClient, DEFAULT_API_V4_URL};
use dedust_api_client::unwrap_v4_response;
use dedust_api_client::v4::{PoolsParams, PoolsResponse, V4Request};
use serde_json::json;
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

#[test]
fn test_pools_params_match_frontend_wire_contract() -> Result<()> {
    let params = PoolsParams::new().with_wallet_address("wallet".to_owned());

    assert_eq!(
        serde_json::to_value(params)?,
        json!({
            "offset": 0,
            "limit": 25,
            "sort_by": "volume_24h",
            "sort_direction": "desc",
            "wallet_address": "wallet"
        })
    );
    Ok(())
}

#[test]
fn test_pools_response_accepts_optional_pool_enrichment() -> Result<()> {
    let response: PoolsResponse = serde_json::from_value(json!({
        "assets_metadata": {
            "native": {
                "decimals": 9,
                "description": "",
                "image_url": "https://cdn.dedust.io/images/native.webp",
                "name": "Toncoin",
                "ticker": "TON",
                "usd_price": "1.25"
            }
        },
        "pool_rows": [{
            "apr": "3.5",
            "apr_fees": "3.5",
            "apr_rewards": "0",
            "assets": ["native", "jetton:0:asset"],
            "fees_24h_usd": "1.1",
            "pools": [{
                "address": "0:pool",
                "apr": "3.5",
                "apr_fees": "3.5",
                "apr_rewards": "0",
                "assets": ["native", "jetton:0:asset"],
                "creator_fee": "0",
                "dex": "dedust",
                "fee_in_assets": ["native", "jetton:0:asset"],
                "fees": ["1", "2"],
                "fees_24h_usd": "1.1",
                "last_activity_at": null,
                "lp_fee": "8",
                "protocol_fee": "2",
                "reserves": ["100", "200"],
                "total_supply": "150",
                "trade_fee": "10",
                "tvl_usd": "1000",
                "type": "cpmm_v1",
                "verified": true,
                "volume": ["10", "20"],
                "volume_24h_usd": "100"
            }],
            "pools_limit": 5,
            "pools_offset": 0,
            "total_pools": 1,
            "tvl_usd": "1000",
            "volume_24h_usd": "100"
        }],
        "total_count": 1
    }))?;

    let pool = &response.pool_rows[0].pools[0];
    assert_eq!(pool.pool_type, "cpmm_v1");
    assert_eq!(pool.last_activity_at, None);
    assert_eq!(pool.rewards, None);
    assert_eq!(response.assets_metadata["native"].ticker, "TON");
    Ok(())
}
