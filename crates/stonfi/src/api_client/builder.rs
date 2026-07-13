use crate::api_client::{StonfiApiClient, DEFAULT_API_URL, DEFAULT_API_V1_URL};
use crate::export::ExportApiClient;
use crate::v1::V1ApiClient;
use api_clients_core::{ApiClientsResult, Executor};
use derive_setters::Setters;
use std::sync::Arc;

#[derive(Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Builder {
    api_url: String,
    export_api_url: String,
    executor: Option<Arc<Executor>>,
    export_executor: Option<Arc<Executor>>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            api_url: DEFAULT_API_V1_URL.to_string(),
            export_api_url: DEFAULT_API_URL.to_string(),
            executor: None,
            export_executor: None,
        }
    }

    pub fn build(self) -> ApiClientsResult<StonfiApiClient> {
        let executor = match self.executor {
            Some(executor) => executor,
            None => Executor::builder(self.api_url).build()?.into(),
        };
        let export_executor = match self.export_executor {
            Some(executor) => executor,
            None => Executor::builder(self.export_api_url).build()?.into(),
        };

        let v1 = V1ApiClient::new(executor);
        let export = ExportApiClient::new(export_executor);
        Ok(StonfiApiClient { v1, export })
    }
}
