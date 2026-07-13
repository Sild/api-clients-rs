mod builder;

use crate::api_client::builder::Builder;
use crate::export::ExportApiClient;
use crate::v1::V1ApiClient;
pub const DEFAULT_API_URL: &str = "https://api.ston.fi";
pub const DEFAULT_API_V1_URL: &str = "https://api.ston.fi/v1";

#[derive(Clone)]
#[non_exhaustive]
pub struct StonfiApiClient {
    pub v1: V1ApiClient,
    pub export: ExportApiClient,
}

impl StonfiApiClient {
    pub fn builder() -> Builder { Builder::new() }
}
