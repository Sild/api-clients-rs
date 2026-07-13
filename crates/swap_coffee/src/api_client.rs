mod builder;

use crate::api_client::builder::Builder;
use crate::v1::V1ApiClient;

pub const DEFAULT_API_V1_URL: &str = "https://backend.swap.coffee/v1";

#[derive(Clone)]
#[non_exhaustive]
pub struct SwapCoffeeApiClient {
    pub v1: V1ApiClient,
}

impl SwapCoffeeApiClient {
    pub fn builder() -> Builder { Builder::new() }
}
