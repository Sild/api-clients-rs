use swap_coffee_api_client::api_client::SwapCoffeeApiClient;
use swap_coffee_api_client::v1::{Address, Dexes, Pool, PoolsResponse, Token, V1Request};

#[test]
fn test_client_exposes_v1_executor() -> anyhow::Result<()> {
    let client = SwapCoffeeApiClient::builder().build()?;
    let future = client.v1.exec(V1Request::Assets);
    drop(future);
    Ok(())
}

#[test]
fn test_request_params_support_default_setter_construction() {
    let dexes = Dexes::default().with_dexes("stonfi_v2".to_string());

    assert_eq!(dexes.dexes, "stonfi_v2");
}

#[test]
fn test_response_models_support_default_setter_construction() {
    let token = Token::default().with_address(Address::default().with_address("asset".to_string()));
    let pool = Pool::default().with_tokens(vec![token]);
    let response = PoolsResponse::default().with_pools(vec![]);

    assert_eq!(pool.tokens[0].address.address, "asset");
    assert_eq!(response.pools.len(), 0);
}
