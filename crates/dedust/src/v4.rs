mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

/// Executes typed requests against the DeDust API v4 pool registry.
#[derive(Clone)]
pub struct V4ApiClient {
    executor: Arc<Executor>,
}

impl V4ApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    /// Execute a DeDust v4 pool-registry request and return its matching response variant.
    ///
    /// # Errors
    ///
    /// Returns an error when transport, status handling, or response deserialization fails.
    pub async fn exec<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<V4Response>
    where
        REQUEST: Into<V4Request>,
    {
        let response = match request.into() {
            V4Request::AllClassicPools => {
                V4Response::AllClassicPools(self.executor.exec_get("get_pools_allclassic").await?)
            }
            V4Request::AllStablePools => {
                V4Response::AllStablePools(self.executor.exec_get("get_pools_allstable").await?)
            }
            V4Request::AllCpmmPools => V4Response::AllCpmmPools(self.executor.exec_get("get_pools_allcpmm").await?),
            V4Request::AllUranusPools => {
                V4Response::AllUranusPools(self.executor.exec_get("get_pools_alluranus").await?)
            }
        };
        Ok(response)
    }
}
