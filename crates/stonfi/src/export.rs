mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsResult, Executor};
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

#[derive(Clone)]
pub struct ExportApiClient {
    executor: Arc<Executor>,
}

impl ExportApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    pub async fn exec<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<ExportResponse>
    where
        REQUEST: Into<ExportRequest>,
    {
        let request = request.into();
        let response = match &request {
            ExportRequest::Cmc => ExportResponse::Cmc(self.executor.exec_get("export/cmc/v1").await?),
            ExportRequest::DexscreenerLatestBlock => ExportResponse::DexscreenerLatestBlock(
                self.executor.exec_get("export/dexscreener/v1/latest-block").await?,
            ),
            ExportRequest::DexscreenerAsset(address) => ExportResponse::DexscreenerAsset(
                self.executor.exec_get(&format!("export/dexscreener/v1/asset/{address}")).await?,
            ),
            ExportRequest::DexscreenerPair(address) => ExportResponse::DexscreenerPair(
                self.executor.exec_get(&format!("export/dexscreener/v1/pair/{address}")).await?,
            ),
            ExportRequest::DexscreenerEvents(params) => ExportResponse::DexscreenerEvents(
                self.executor.exec_get_extra("export/dexscreener/v1/events", params, &[]).await?,
            ),
        };
        Ok(response)
    }
}
