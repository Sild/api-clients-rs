use std::vec;

use anyhow::Result;
use swap_coffee_api_client::api_client::SwapCoffeeApiClient;
use swap_coffee_api_client::unwrap_response;
use swap_coffee_api_client::v1::Dexes;
use swap_coffee_api_client::v1::V1Request;

fn init_env() -> SwapCoffeeApiClient {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    SwapCoffeeApiClient::builder().build().unwrap()
}

#[tokio::test]
async fn test_assets() -> Result<()> {
    let client = init_env();
    let request = V1Request::Assets;
    let response = unwrap_response!(Assets, client.v1.exec(&request).await?)?;
    assert_ne!(response, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pools() -> Result<()> {
    let client = init_env();
    let response = unwrap_response!(Pools, client.v1.exec(Dexes::new("coffee")).await?)?;
    log::debug!("response: {:?}", response[0].pools[0]);
    assert_ne!(response[0].pools, vec![]);
    Ok(())
}
