mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsError, ApiClientsResult, Executor};
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

const JSON_CONTENT_TYPE: (&str, &str) = ("content-type", "application/json");

/// Executes typed requests against the DeDust API v4 pool-discovery service.
#[derive(Clone)]
pub struct V4ApiClient {
    executor: Arc<Executor>,
}

impl V4ApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    /// Load a filtered page of enriched pool rows used by the DeDust web application.
    ///
    /// # Errors
    ///
    /// Returns an error when transport, status handling, request serialization, or
    /// response deserialization fails.
    pub async fn pools(&self, params: &PoolsParams) -> ApiClientsResult<PoolsResponse> {
        let headers = [(JSON_CONTENT_TYPE.0.to_owned(), JSON_CONTENT_TYPE.1.to_owned())];
        self.executor.exec_post_body("get_pools", params, &headers).await
    }

    /// Load every enriched pool row matching the supplied filters.
    ///
    /// Pagination starts at offset zero and uses `params.limit` as the page size.
    /// Pages are loaded sequentially and no partial response is returned if a
    /// request fails or the upstream service returns an empty page before
    /// `total_count` rows have been loaded.
    ///
    /// # Errors
    ///
    /// Returns an error when any page request fails, a page cannot be deserialized,
    /// or the upstream pagination response is inconsistent.
    pub async fn all_pools(&self, params: &PoolsParams) -> ApiClientsResult<PoolsResponse> {
        let mut page_params = params.clone().with_offset(0);
        let mut response = PoolsResponse::default();
        let mut expected_total = None;

        loop {
            let page = self.pools(&page_params).await?;
            match append_pools_page(&mut response, &mut expected_total, page)? {
                Some(next_offset) => page_params.offset = next_offset,
                None => return Ok(response),
            }
        }
    }

    /// Execute a DeDust v4 registry request and return its matching response variant.
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

fn append_pools_page(
    response: &mut PoolsResponse,
    expected_total: &mut Option<u32>,
    mut page: PoolsResponse,
) -> ApiClientsResult<Option<u32>> {
    let page_row_count = page.pool_rows.len();
    let total_count = match *expected_total {
        Some(total_count) if page.total_count != total_count => {
            return Err(ApiClientsError::UnexpectedResponse(format!(
                "DeDust pool total changed from {total_count} to {} during pagination",
                page.total_count
            )))
        }
        Some(total_count) => total_count,
        None => {
            *expected_total = Some(page.total_count);
            response.total_count = page.total_count;
            page.total_count
        }
    };

    let loaded_count = response
        .pool_rows
        .len()
        .checked_add(page_row_count)
        .ok_or_else(|| ApiClientsError::Internal("DeDust pool row count exceeds usize".to_owned()))?;
    let loaded_count = u32::try_from(loaded_count)
        .map_err(|_| ApiClientsError::Internal("DeDust pool row count exceeds u32".to_owned()))?;
    if loaded_count > total_count {
        return Err(ApiClientsError::UnexpectedResponse(format!(
            "DeDust returned {loaded_count} pool rows for declared total {total_count}"
        )));
    }
    if page_row_count == 0 && loaded_count < total_count {
        return Err(ApiClientsError::UnexpectedResponse(format!(
            "DeDust returned an empty pool page after {loaded_count} of {total_count} rows"
        )));
    }

    response.assets_metadata.append(&mut page.assets_metadata);
    response.pool_rows.append(&mut page.pool_rows);
    Ok((loaded_count < total_count).then_some(loaded_count))
}

#[cfg(test)]
mod tests {
    use super::{append_pools_page, PoolRow, PoolsResponse};

    #[test]
    fn test_append_pools_page_advances_until_total_count() -> anyhow::Result<()> {
        let mut response = PoolsResponse::default();
        let first_page =
            PoolsResponse::default().with_pool_rows(vec![PoolRow::default(), PoolRow::default()]).with_total_count(3);
        let second_page = PoolsResponse::default().with_pool_rows(vec![PoolRow::default()]).with_total_count(3);
        let mut expected_total = None;

        assert_eq!(append_pools_page(&mut response, &mut expected_total, first_page)?, Some(2));
        assert_eq!(append_pools_page(&mut response, &mut expected_total, second_page)?, None);
        assert_eq!(response.pool_rows.len(), 3);
        Ok(())
    }

    #[test]
    fn test_append_pools_page_rejects_incomplete_empty_page() -> anyhow::Result<()> {
        let mut response = PoolsResponse::default();
        let empty_page = PoolsResponse::default().with_total_count(1);
        let mut expected_total = None;

        let error = match append_pools_page(&mut response, &mut expected_total, empty_page) {
            Ok(_) => anyhow::bail!("empty page unexpectedly succeeded"),
            Err(error) => error,
        };

        assert!(matches!(error, api_clients_core::ApiClientsError::UnexpectedResponse(_)));
        Ok(())
    }

    #[test]
    fn test_append_pools_page_rejects_changed_total() -> anyhow::Result<()> {
        let mut response = PoolsResponse::default();
        let first_page = PoolsResponse::default().with_pool_rows(vec![PoolRow::default()]).with_total_count(2);
        let changed_page = PoolsResponse::default().with_pool_rows(vec![PoolRow::default()]).with_total_count(1);
        let mut expected_total = None;

        assert_eq!(append_pools_page(&mut response, &mut expected_total, first_page)?, Some(1));
        let error = match append_pools_page(&mut response, &mut expected_total, changed_page) {
            Ok(_) => anyhow::bail!("changed total unexpectedly succeeded"),
            Err(error) => error,
        };

        assert!(matches!(error, api_clients_core::ApiClientsError::UnexpectedResponse(_)));
        assert_eq!(response.pool_rows.len(), 1);
        Ok(())
    }
}
