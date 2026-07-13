use dedust_api_client::api_client::DedustApiClient;
use dedust_api_client::v2::{Asset, Pool, PoolAsset, RoutingPlanParams, V2Request};

#[test]
fn test_client_exposes_v2_executor() -> anyhow::Result<()> {
    let client = DedustApiClient::builder().build()?;
    let future = client.v2.exec(V2Request::Assets);
    drop(future);
    Ok(())
}

#[test]
fn test_request_params_support_default_setter_construction() {
    let params = RoutingPlanParams::default()
        .with_from("native".to_string())
        .with_to("jetton:asset".to_string())
        .with_amount("1000".to_string());

    assert_eq!(params.from, "native");
    assert_eq!(params.to, "jetton:asset");
    assert_eq!(params.amount, "1000");
}

#[test]
fn test_response_models_support_default_setter_construction() {
    let asset = Asset::default().with_symbol("TON".to_string());
    let pool = Pool::default().with_assets(vec![PoolAsset::default().with_asset_type("native".to_string())]);

    assert_eq!(asset.symbol, "TON");
    assert_eq!(pool.assets[0].asset_type, "native");
}
