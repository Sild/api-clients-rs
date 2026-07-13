mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsResult, Executor};
use serde::Serialize;
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

#[derive(Clone)]
pub struct ApiClient {
    executor: Arc<Executor>,
}

impl ApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    pub async fn exec<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<Response>
    where
        REQUEST: Into<Request>,
    {
        let response = match request.into() {
            Request::Pools => Response::Pools(self.fetch_all_pool_pages().await?),
        };
        Ok(response)
    }

    async fn fetch_all_pool_pages(&self) -> ApiClientsResult<Vec<PoolInfo>> {
        let mut page_number = 1_i64;
        let mut total_pages: Option<i64> = None;
        let mut aggregated_pages = Vec::new();

        loop {
            let params = PaginationParams { page: page_number };
            let PaginatedResponse {
                page: page_info,
                result: pools,
            } = self.executor.exec_get_extra("pools", &params, &[]).await?;

            let page_count = match total_pages {
                Some(count) => count,
                None => {
                    let computed = total_pages_from(&page_info);
                    total_pages = Some(computed);
                    computed
                }
            };

            let is_empty_page = pools.is_empty();
            aggregated_pages.extend_from_slice(&pools);

            if page_number >= page_count || is_empty_page {
                break;
            }

            page_number += 1;
        }

        Ok(aggregated_pages)
    }
}

#[derive(Serialize)]
struct PaginationParams {
    page: i64,
}

fn total_pages_from(page_info: &PageInfo) -> i64 {
    if page_info.size <= 0 {
        return 1;
    }

    let pages = (page_info.total + page_info.size - 1) / page_info.size;
    pages.max(1)
}
