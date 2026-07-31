use anyhow::Result;
use api_clients_core::Executor;
use dedust_api_client::api_client::{DedustApiClient, DEFAULT_ASSETS_URL};
use dedust_api_client::assets::AssetsRequest;
use dedust_api_client::unwrap_assets_response;
use std::sync::Arc;
use std::time::Duration;

fn init_env() -> Result<DedustApiClient> {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    let executor = Executor::builder(DEFAULT_ASSETS_URL).with_timeout(Duration::from_secs(60)).build()?;
    Ok(DedustApiClient::builder().with_assets_executor(Arc::new(executor)).build()?)
}

#[tokio::test]
async fn test_asset_list() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_assets_response!(List, client.assets.exec(AssetsRequest::List).await?)?;

    assert!(!response.is_empty());
    Ok(())
}
