mod builder;

use crate::api_client::builder::Builder;
use crate::export::ExportApiClient;
use crate::v1::V1ApiClient;
/// Default STON.fi base URL used for public export feeds.
pub const DEFAULT_API_URL: &str = "https://api.ston.fi";
/// Default base URL for STON.fi API v1.
pub const DEFAULT_API_V1_URL: &str = "https://api.ston.fi/v1";

/// STON.fi service client with API v1 and public export child clients.
#[derive(Clone)]
#[non_exhaustive]
pub struct StonfiApiClient {
    /// STON.fi API v1 execution client.
    pub v1: V1ApiClient,
    /// STON.fi public export-feed execution client.
    pub export: ExportApiClient,
}

impl StonfiApiClient {
    /// Start configuring a STON.fi client with the default endpoints.
    pub fn builder() -> Builder { Builder::new() }
}
