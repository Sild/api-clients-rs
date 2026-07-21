use crate::api::ApiClient;
use crate::api_client::{StonksApiClient, DEFAULT_API_URL};
use api_clients_core::{ApiClientsResult, Executor};
use derive_setters::Setters;
use std::sync::Arc;

/// Builder for [`StonksApiClient`].
#[derive(Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Builder {
    api_url: String,
    executor: Option<Arc<Executor>>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            api_url: DEFAULT_API_URL.to_string(),
            executor: None,
        }
    }

    /// Build the configured Stonks client.
    ///
    /// # Errors
    ///
    /// Returns an error if the shared executor cannot be constructed.
    pub fn build(self) -> ApiClientsResult<StonksApiClient> {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.api_url).build()?.into(),
        };

        let api = ApiClient::new(executor);
        Ok(StonksApiClient { api })
    }
}
