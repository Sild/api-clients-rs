use anyhow::{bail, Result};
use dedust_api_client::assets::Asset;

#[test]
fn test_asset_deserializes_complete_observed_wire_shape() -> Result<()> {
    let asset: Asset = serde_json::from_str(
        r#"{
            "type": "jetton",
            "address": "EQAsset",
            "name": "Example",
            "symbol": "EX",
            "image": "https://assets.dedust.io/images/example.webp",
            "decimals": 9,
            "aliased": true,
            "buy_tax": 300,
            "sell_tax": 500,
            "description": "https://example.com/description.mp3",
            "discovery": "EQDiscovery",
            "legacyBridge": true,
            "source": {
                "chain": "eip155:1",
                "address": "0x1234",
                "bridge": "ton-bridge",
                "symbol": "EX",
                "name": "Example Source"
            },
            "token_page": {
                "banners": {
                    "small": "https://assets.dedust.io/images/example-small.webp",
                    "large": "https://assets.dedust.io/images/example-large.webp"
                },
                "description": {
                    "title": "Example title",
                    "text": "Example text",
                    "banner": "https://assets.dedust.io/images/example-description.webp"
                }
            }
        }"#,
    )?;

    assert_eq!(asset.asset_type, "jetton");
    assert_eq!(asset.address.as_deref(), Some("EQAsset"));
    assert_eq!(asset.name, "Example");
    assert_eq!(asset.symbol, "EX");
    assert_eq!(asset.image, "https://assets.dedust.io/images/example.webp");
    assert_eq!(asset.decimals, 9);
    assert_eq!(asset.aliased, Some(true));
    assert_eq!(asset.buy_tax, Some(300));
    assert_eq!(asset.sell_tax, Some(500));
    assert_eq!(asset.description.as_deref(), Some("https://example.com/description.mp3"));
    assert_eq!(asset.discovery.as_deref(), Some("EQDiscovery"));
    assert_eq!(asset.legacy_bridge, Some(true));

    let Some(source) = asset.source else {
        bail!("expected source metadata");
    };
    assert_eq!(source.chain, "eip155:1");
    assert_eq!(source.address, "0x1234");
    assert_eq!(source.bridge, "ton-bridge");
    assert_eq!(source.symbol, "EX");
    assert_eq!(source.name, "Example Source");

    let Some(token_page) = asset.token_page else {
        bail!("expected token-page metadata");
    };
    assert_eq!(token_page.banners.small, "https://assets.dedust.io/images/example-small.webp");
    assert_eq!(token_page.banners.large, "https://assets.dedust.io/images/example-large.webp");
    assert_eq!(token_page.description.title, "Example title");
    assert_eq!(token_page.description.text, "Example text");
    assert_eq!(
        token_page.description.banner.as_deref(),
        Some("https://assets.dedust.io/images/example-description.webp")
    );

    let sparse_asset: Asset = serde_json::from_str(
        r#"{
            "type": "jetton",
            "address": "EQSparse",
            "name": "Sparse Example",
            "symbol": "SPARSE",
            "image": "https://assets.dedust.io/images/sparse.webp",
            "decimals": 9,
            "token_page": {
                "banners": {
                    "small": "https://assets.dedust.io/images/sparse-small.webp",
                    "large": "https://assets.dedust.io/images/sparse-large.webp"
                },
                "description": {
                    "title": "Sparse title",
                    "text": "Sparse text"
                }
            }
        }"#,
    )?;
    assert_eq!(sparse_asset.aliased, None);
    assert_eq!(sparse_asset.buy_tax, None);
    assert_eq!(sparse_asset.sell_tax, None);
    assert_eq!(sparse_asset.description, None);
    assert_eq!(sparse_asset.discovery, None);
    assert_eq!(sparse_asset.legacy_bridge, None);
    assert_eq!(sparse_asset.source, None);
    let Some(sparse_token_page) = sparse_asset.token_page else {
        bail!("expected sparse token-page metadata");
    };
    assert_eq!(sparse_token_page.description.banner, None);
    Ok(())
}
