mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsError, ApiClientsResult, Executor};
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

const ALL_POOLS_PAGE_SIZE: usize = 100;
const ASSETS_PATH: &str = "api/deployments/public-tokens";
const POOLS_PATH: &str = "api/virtual-deployments/non-bonded-tokens";

/// Executes typed requests against the Stonks public API.
#[derive(Clone)]
pub struct ApiClient {
    executor: Arc<Executor>,
}

impl ApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    /// Execute a Stonks request and return its matching response variant.
    ///
    /// `Request::AllPools` loads pages sequentially and returns no partial
    /// result if any page fails.
    ///
    /// # Errors
    ///
    /// Returns an error when request serialization, transport, status handling,
    /// response deserialization, or automatic pagination fails.
    pub async fn exec<REQUEST>(&self, request: REQUEST) -> ApiClientsResult<Response>
    where
        REQUEST: Into<Request>,
    {
        let request = request.into();
        let response = match &request {
            Request::Assets => Response::Assets(self.executor.exec_get(ASSETS_PATH).await?),
            Request::Pools(params) => Response::Pools(self.load_pool_page(params).await?),
            Request::AllPools => Response::AllPools(self.load_all_pools().await?),
        };
        Ok(response)
    }

    async fn load_pool_page(&self, params: &PoolsParams) -> ApiClientsResult<Vec<String>> {
        self.executor.exec_get_extra(POOLS_PATH, params, &[]).await
    }

    async fn load_all_pools(&self) -> ApiClientsResult<Vec<String>> {
        let mut pools = Vec::new();
        let mut page = 0_usize;

        loop {
            let params = PoolsParams::new(page, ALL_POOLS_PAGE_SIZE);
            let page_pools = self.load_pool_page(&params).await?;

            if append_pool_page(&mut pools, &mut page, page_pools)? {
                return Ok(pools);
            }
        }
    }
}

fn append_pool_page(pools: &mut Vec<String>, page: &mut usize, mut page_pools: Vec<String>) -> ApiClientsResult<bool> {
    let is_last_page = page_pools.len() < ALL_POOLS_PAGE_SIZE;
    let next_page = if is_last_page {
        None
    } else {
        Some(
            page.checked_add(1)
                .ok_or_else(|| ApiClientsError::Internal("Stonks pool pagination page overflow".to_string()))?,
        )
    };

    pools.append(&mut page_pools);
    if let Some(next_page) = next_page {
        *page = next_page;
    }
    Ok(is_last_page)
}

#[cfg(test)]
mod tests {
    use super::{append_pool_page, ALL_POOLS_PAGE_SIZE};

    #[test]
    fn test_full_pool_page_advances_pagination() -> anyhow::Result<()> {
        let mut pools = Vec::new();
        let mut page = 0;
        let page_pools = (0..ALL_POOLS_PAGE_SIZE).map(|index| index.to_string()).collect();

        let is_last_page = append_pool_page(&mut pools, &mut page, page_pools)?;

        assert!(!is_last_page);
        assert_eq!(page, 1);
        assert_eq!(pools.len(), ALL_POOLS_PAGE_SIZE);
        Ok(())
    }

    #[test]
    fn test_short_pool_page_finishes_without_reordering_or_deduplication() -> anyhow::Result<()> {
        let mut pools = vec!["first".to_string()];
        let mut page = 6;

        let is_last_page =
            append_pool_page(&mut pools, &mut page, vec!["duplicate".to_string(), "duplicate".to_string()])?;

        assert!(is_last_page);
        assert_eq!(page, 6);
        assert_eq!(pools, vec!["first", "duplicate", "duplicate"]);
        Ok(())
    }

    #[test]
    fn test_pool_page_overflow_returns_error_without_appending() {
        let mut pools = vec!["existing".to_string()];
        let mut page = usize::MAX;
        let page_pools = vec!["new".to_string(); ALL_POOLS_PAGE_SIZE];

        let result = append_pool_page(&mut pools, &mut page, page_pools);

        assert!(result.is_err());
        assert_eq!(page, usize::MAX);
        assert_eq!(pools, vec!["existing"]);
    }
}
