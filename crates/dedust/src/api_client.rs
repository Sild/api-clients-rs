mod builder;

use crate::api_client::builder::Builder;
use crate::assets::AssetsApiClient;
use crate::v2::V2ApiClient;
use crate::v4::V4ApiClient;

/// Default base URL for the DeDust asset registry.
pub const DEFAULT_ASSETS_URL: &str = "https://assets.dedust.io";

/// Default base URL for DeDust API v2.
pub const DEFAULT_API_V2_URL: &str = "https://api.dedust.io/v2";

/// Default base URL for the DeDust API v4 pool-discovery service.
pub const DEFAULT_API_V4_URL: &str = "https://mainnet.api.dedust.io/v4/api";

/// DeDust service client with child clients for each upstream API surface.
#[derive(Clone)]
#[non_exhaustive]
pub struct DedustApiClient {
    /// DeDust asset-registry execution client.
    pub assets: AssetsApiClient,
    /// DeDust API v2 execution client.
    pub v2: V2ApiClient,
    /// DeDust API v4 pool-discovery execution client.
    pub v4: V4ApiClient,
}

impl DedustApiClient {
    /// Start configuring a DeDust client with the default asset, v2, and v4 endpoints.
    pub fn builder() -> Builder { Builder::new() }
}
