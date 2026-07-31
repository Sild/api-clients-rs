//! Raw DeDust asset-registry wire types.

use derive_setters::Setters;
use serde_derive::{Deserialize, Serialize};

/// Raw metadata for an asset listed by DeDust.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Asset {
    /// Upstream asset kind, such as `native` or `jetton`.
    #[serde(rename = "type")]
    pub asset_type: String,
    /// Friendly TON jetton-master address; absent for the native asset.
    pub address: Option<String>,
    /// Display name supplied by DeDust.
    pub name: String,
    /// Display symbol supplied by DeDust.
    pub symbol: String,
    /// Absolute asset-image URL.
    pub image: String,
    /// Number of decimal places used by the asset.
    pub decimals: i64,
    /// Raw alias flag supplied by DeDust.
    pub aliased: Option<bool>,
    /// Raw buy-tax value supplied by DeDust.
    pub buy_tax: Option<u16>,
    /// Raw sell-tax value supplied by DeDust.
    pub sell_tax: Option<u16>,
    /// Optional upstream description value, currently used for media URLs.
    pub description: Option<String>,
    /// Optional friendly TON address used for DeDust discovery metadata.
    pub discovery: Option<String>,
    /// Whether the asset originated from the legacy TON bridge.
    #[serde(rename = "legacyBridge")]
    pub legacy_bridge: Option<bool>,
    /// Optional source-chain and bridge metadata.
    pub source: Option<AssetSource>,
    /// Optional content shown on the DeDust token page.
    pub token_page: Option<TokenPage>,
}

/// Raw source-chain metadata for a bridged or externally sourced asset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AssetSource {
    /// CAIP-style source-chain identifier.
    pub chain: String,
    /// Source-chain token address, when supplied.
    pub address: String,
    /// Bridge identifier, when supplied.
    pub bridge: String,
    /// Source-chain token symbol.
    pub symbol: String,
    /// Source-chain token name.
    pub name: String,
}

/// Raw DeDust token-page content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TokenPage {
    /// Responsive token-page banner images.
    pub banners: TokenPageBanners,
    /// Token-page descriptive content.
    pub description: TokenPageDescription,
}

/// Raw token-page banner URLs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TokenPageBanners {
    /// Small-layout banner URL.
    pub small: String,
    /// Large-layout banner URL.
    pub large: String,
}

/// Raw token-page description fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TokenPageDescription {
    /// Description heading.
    pub title: String,
    /// Description body.
    pub text: String,
    /// Optional description banner URL.
    pub banner: Option<String>,
}
