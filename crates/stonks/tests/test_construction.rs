use stonks_api_client::api::{Asset, PoolsParams, Request};
use stonks_api_client::api_client::StonksApiClient;

#[test]
fn test_client_exposes_api_executor() -> anyhow::Result<()> {
    let client = StonksApiClient::builder().build()?;
    let assets = client.api.exec(Request::Assets);
    let pools = client.api.exec(PoolsParams::new(0, 10));
    let all_pools = client.api.exec(Request::AllPools);

    drop((assets, pools, all_pools));
    Ok(())
}

#[test]
fn test_pools_params_serialize_to_upstream_query_names() -> anyhow::Result<()> {
    let params = PoolsParams::new(2, 50);

    assert_eq!(serde_qs::to_string(&params)?, "page=2&size=50");
    Ok(())
}

#[test]
fn test_asset_deserializes_all_upstream_fields() -> anyhow::Result<()> {
    let asset: Asset = serde_json::from_value(serde_json::json!({
        "symbol": "TOKEN",
        "address": "EQAddress",
        "buyTax": 3,
        "sellTax": 7
    }))?;

    assert_eq!(asset.symbol, "TOKEN");
    assert_eq!(asset.address, "EQAddress");
    assert_eq!(asset.buy_tax, 3);
    assert_eq!(asset.sell_tax, 7);
    Ok(())
}

#[test]
fn test_response_models_support_default_setter_construction() {
    let asset = Asset::default()
        .with_symbol("TOKEN".to_string())
        .with_address("EQAddress".to_string())
        .with_buy_tax(3)
        .with_sell_tax(7);
    let params = PoolsParams::default().with_page(2).with_size(50);

    assert_eq!(asset.symbol, "TOKEN");
    assert_eq!(asset.address, "EQAddress");
    assert_eq!(asset.buy_tax, 3);
    assert_eq!(asset.sell_tax, 7);
    assert_eq!(params, PoolsParams::new(2, 50));
}
