use anyhow::{Context, Result};
use stonks_api_client::api::{Request, VirtualPoolAddressesParams};
use stonks_api_client::api_client::StonksApiClient;
use stonks_api_client::unwrap_response;

fn init_client() -> Result<StonksApiClient> { Ok(StonksApiClient::builder().build()?) }

#[tokio::test]
async fn test_public_tokens() -> Result<()> {
    let client = init_client()?;
    let public_tokens = unwrap_response!(PublicTokens, client.api.exec(Request::PublicTokens).await?)?;
    let public_token = public_tokens.first().context("Stonks returned no public tokens")?;

    assert!(!public_token.symbol.is_empty());
    assert!(!public_token.address.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_virtual_pool_addresses_page() -> Result<()> {
    let client = init_client()?;
    let addresses =
        unwrap_response!(VirtualPoolAddresses, client.api.exec(VirtualPoolAddressesParams::new(0, 1)).await?)?;
    let address = addresses.first().context("Stonks returned no Virtual Pool addresses")?;

    assert!(addresses.len() <= 1);
    assert!(!address.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_all_virtual_pool_addresses() -> Result<()> {
    let client = init_client()?;
    let addresses =
        unwrap_response!(AllVirtualPoolAddresses, client.api.exec(Request::AllVirtualPoolAddresses).await?)?;

    assert!(!addresses.is_empty());
    assert!(addresses.iter().all(|address| !address.is_empty()));
    Ok(())
}
