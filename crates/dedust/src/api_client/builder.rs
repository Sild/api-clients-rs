use crate::api_client::{DedustApiClient, DEFAULT_API_V2_URL};
use crate::v2::V2ApiClient;
use api_clients_core::{ApiClientsResult, Executor};
use derive_setters::Setters;
use std::sync::Arc;

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
            api_url: DEFAULT_API_V2_URL.to_string(),
            executor: None,
        }
    }

    pub fn build(self) -> ApiClientsResult<DedustApiClient> {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.api_url).build()?.into(),
        };

        let v2 = V2ApiClient::new(executor);
        Ok(DedustApiClient { v2 })
    }
}
