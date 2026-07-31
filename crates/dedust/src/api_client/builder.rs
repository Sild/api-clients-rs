use crate::api_client::{DedustApiClient, DEFAULT_API_V2_URL, DEFAULT_API_V4_URL, DEFAULT_ASSETS_URL};
use crate::assets::AssetsApiClient;
use crate::v2::V2ApiClient;
use crate::v4::V4ApiClient;
use api_clients_core::{ApiClientsResult, Executor};
use derive_setters::Setters;
use std::sync::Arc;

/// Builder for [`DedustApiClient`].
#[derive(Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Builder {
    api_url: String,
    assets_url: String,
    v4_url: String,
    executor: Option<Arc<Executor>>,
    assets_executor: Option<Arc<Executor>>,
    v4_executor: Option<Arc<Executor>>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            api_url: DEFAULT_API_V2_URL.to_string(),
            assets_url: DEFAULT_ASSETS_URL.to_string(),
            v4_url: DEFAULT_API_V4_URL.to_string(),
            executor: None,
            assets_executor: None,
            v4_executor: None,
        }
    }

    /// Build the configured DeDust client.
    ///
    /// # Errors
    ///
    /// Returns an error if any shared executor cannot be constructed.
    pub fn build(self) -> ApiClientsResult<DedustApiClient> {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.api_url).build()?.into(),
        };
        let assets_executor = match self.assets_executor {
            Some(executor) => executor,
            None => Executor::builder(self.assets_url).build()?.into(),
        };
        let v4_executor = match self.v4_executor {
            Some(executor) => executor,
            None => Executor::builder(self.v4_url).build()?.into(),
        };

        let assets = AssetsApiClient::new(assets_executor);
        let v2 = V2ApiClient::new(executor);
        let v4 = V4ApiClient::new(v4_executor);
        Ok(DedustApiClient { assets, v2, v4 })
    }
}
