use stonks_api_client::api::{PublicToken, Request, VirtualPoolAddressesParams};
use stonks_api_client::api_client::StonksApiClient;

#[test]
fn test_client_exposes_api_executor() -> anyhow::Result<()> {
    let client = StonksApiClient::builder().build()?;
    let public_tokens = client.api.exec(Request::PublicTokens);
    let virtual_pool_addresses = client.api.exec(VirtualPoolAddressesParams::new(0, 10));
    let all_virtual_pool_addresses = client.api.exec(Request::AllVirtualPoolAddresses);

    drop((public_tokens, virtual_pool_addresses, all_virtual_pool_addresses));
    Ok(())
}

#[test]
fn test_virtual_pool_addresses_params_serialize_to_upstream_query_names() -> anyhow::Result<()> {
    let params = VirtualPoolAddressesParams::new(2_u32, 50_u32);

    assert_eq!(serde_qs::to_string(&params)?, "page=2&size=50");
    Ok(())
}

#[test]
fn test_public_token_deserializes_all_upstream_fields() -> anyhow::Result<()> {
    let public_token: PublicToken = serde_json::from_value(serde_json::json!({
        "symbol": "TOKEN",
        "address": "EQAddress",
        "buyTax": 3,
        "sellTax": 7
    }))?;

    assert_eq!(public_token.symbol, "TOKEN");
    assert_eq!(public_token.address, "EQAddress");
    assert_eq!(public_token.buy_tax, 3);
    assert_eq!(public_token.sell_tax, 7);
    Ok(())
}

#[test]
fn test_response_models_support_default_setter_construction() {
    let public_token = PublicToken::default()
        .with_symbol("TOKEN".to_string())
        .with_address("EQAddress".to_string())
        .with_buy_tax(3)
        .with_sell_tax(7);
    let params = VirtualPoolAddressesParams::default().with_page(2).with_size(50);

    assert_eq!(public_token.symbol, "TOKEN");
    assert_eq!(public_token.address, "EQAddress");
    assert_eq!(public_token.buy_tax, 3);
    assert_eq!(public_token.sell_tax, 7);
    assert_eq!(params, VirtualPoolAddressesParams::new(2, 50));
}
