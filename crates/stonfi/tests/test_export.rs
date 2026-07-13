use anyhow::Result;
use api_clients_core::Executor;
use std::sync::Arc;
use std::time::Duration;
use stonfi_api_client::api_client::StonfiApiClient;
use stonfi_api_client::api_client::DEFAULT_API_URL;
use stonfi_api_client::export::*;

fn init_env() -> Result<StonfiApiClient> {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    let executor = Executor::builder(DEFAULT_API_URL).with_timeout(Duration::from_secs(60)).build()?;
    Ok(StonfiApiClient::builder().with_export_executor(Arc::new(executor)).build()?)
}

#[tokio::test]
async fn test_export_cmc() -> Result<()> {
    let client = init_env()?;
    let response = client.export.exec(ExportRequest::Cmc).await?;

    match response {
        ExportResponse::Cmc(stats) => assert!(!stats.is_empty()),
        _ => anyhow::bail!("unexpected export response"),
    }
    Ok(())
}

#[tokio::test]
async fn test_export_dexscreener_latest_block() -> Result<()> {
    let client = init_env()?;
    let response = client.export.exec(ExportRequest::DexscreenerLatestBlock).await?;

    match response {
        ExportResponse::DexscreenerLatestBlock(block) => assert!(block.block.block_number > 0),
        _ => anyhow::bail!("unexpected export response"),
    }
    Ok(())
}

#[tokio::test]
async fn test_export_dexscreener_asset() -> Result<()> {
    let client = init_env()?;
    let address = "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c";
    let response = client.export.exec(ExportRequest::DexscreenerAsset(address.to_string())).await?;

    match response {
        ExportResponse::DexscreenerAsset(asset) => assert_eq!(asset.asset.id, address),
        _ => anyhow::bail!("unexpected export response"),
    }
    Ok(())
}

#[tokio::test]
async fn test_export_dexscreener_pair() -> Result<()> {
    let client = init_env()?;
    let address = "EQCGScrZe1xbyWqWDvdI6mzP-GAcAWFv6ZXuaJOuSqemxku4";
    let response = client.export.exec(ExportRequest::DexscreenerPair(address.to_string())).await?;

    match response {
        ExportResponse::DexscreenerPair(pair) => assert_eq!(pair.pool.id, address),
        _ => anyhow::bail!("unexpected export response"),
    }
    Ok(())
}

#[tokio::test]
async fn test_export_dexscreener_events() -> Result<()> {
    let client = init_env()?;
    let latest = match client.export.exec(ExportRequest::DexscreenerLatestBlock).await? {
        ExportResponse::DexscreenerLatestBlock(block) => block.block.block_number,
        _ => anyhow::bail!("unexpected export response"),
    };

    let response = client.export.exec(DexscreenerEventsParams::new(latest.saturating_sub(2), latest)).await?;

    match response {
        ExportResponse::DexscreenerEvents(events) => assert!(events.events.len() <= 2_000),
        _ => anyhow::bail!("unexpected export response"),
    }
    Ok(())
}
