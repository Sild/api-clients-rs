mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

#[derive(Clone)]
pub struct V1ApiClient {
    executor: Arc<Executor>,
}

impl V1ApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    pub async fn exec<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<V1Response>
    where
        REQUEST: Into<V1Request>,
    {
        let request = request.into();
        let response = match &request {
            V1Request::Assets => V1Response::Assets(self.executor.exec_get("tokens").await?),
            V1Request::Pools(dexes) => V1Response::Pools(self.executor.exec_get_extra("pools", dexes, &[]).await?),
        };
        Ok(response)
    }
}
