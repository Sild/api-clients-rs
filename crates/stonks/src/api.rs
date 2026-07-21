mod request;
mod response;
mod types;

use api_clients_core::{ApiClientsError, ApiClientsResult, Executor};
use std::sync::Arc;

pub use request::*;
pub use response::*;
pub use types::*;

const ALL_VIRTUAL_POOL_ADDRESSES_PAGE_SIZE: u32 = 100;
const PUBLIC_TOKENS_PATH: &str = "api/deployments/public-tokens";
const VIRTUAL_POOL_ADDRESSES_PATH: &str = "api/virtual-deployments/non-bonded-tokens";

/// Executes typed requests against the Stonks public API.
#[derive(Clone)]
pub struct ApiClient {
    executor: Arc<Executor>,
}

impl ApiClient {
    pub(crate) fn new(executor: Arc<Executor>) -> Self { Self { executor } }

    /// Execute a Stonks request and return its matching response variant.
    ///
    /// `Request::AllVirtualPoolAddresses` loads pages sequentially and returns
    /// no partial result if any page fails.
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
            Request::PublicTokens => Response::PublicTokens(self.executor.exec_get(PUBLIC_TOKENS_PATH).await?),
            Request::VirtualPoolAddresses(params) => {
                Response::VirtualPoolAddresses(self.load_virtual_pool_addresses_page(params).await?)
            }
            Request::AllVirtualPoolAddresses => {
                Response::AllVirtualPoolAddresses(self.load_all_virtual_pool_addresses().await?)
            }
        };
        Ok(response)
    }

    async fn load_virtual_pool_addresses_page(
        &self,
        params: &VirtualPoolAddressesParams,
    ) -> ApiClientsResult<Vec<String>> {
        self.executor.exec_get_extra(VIRTUAL_POOL_ADDRESSES_PATH, params, &[]).await
    }

    async fn load_all_virtual_pool_addresses(&self) -> ApiClientsResult<Vec<String>> {
        let mut addresses = Vec::new();
        let mut page = 0_u32;

        loop {
            let params = VirtualPoolAddressesParams::new(page, ALL_VIRTUAL_POOL_ADDRESSES_PAGE_SIZE);
            let page_addresses = self.load_virtual_pool_addresses_page(&params).await?;

            if append_virtual_pool_addresses_page(&mut addresses, &mut page, page_addresses)? {
                return Ok(addresses);
            }
        }
    }
}

fn append_virtual_pool_addresses_page(
    addresses: &mut Vec<String>,
    page: &mut u32,
    mut page_addresses: Vec<String>,
) -> ApiClientsResult<bool> {
    let is_last_page = page_addresses.len() < ALL_VIRTUAL_POOL_ADDRESSES_PAGE_SIZE as usize;
    let next_page = if is_last_page {
        None
    } else {
        Some(
            page.checked_add(1)
                .ok_or_else(|| ApiClientsError::Internal("Stonks Virtual Pool address page overflow".to_string()))?,
        )
    };

    addresses.append(&mut page_addresses);
    if let Some(next_page) = next_page {
        *page = next_page;
    }
    Ok(is_last_page)
}

#[cfg(test)]
mod tests {
    use super::{append_virtual_pool_addresses_page, ALL_VIRTUAL_POOL_ADDRESSES_PAGE_SIZE};

    #[test]
    fn test_full_virtual_pool_address_page_advances_pagination() -> anyhow::Result<()> {
        let mut addresses = Vec::new();
        let mut page = 0;
        let page_addresses = (0..ALL_VIRTUAL_POOL_ADDRESSES_PAGE_SIZE).map(|index| index.to_string()).collect();

        let is_last_page = append_virtual_pool_addresses_page(&mut addresses, &mut page, page_addresses)?;

        assert!(!is_last_page);
        assert_eq!(page, 1);
        assert_eq!(addresses.len(), ALL_VIRTUAL_POOL_ADDRESSES_PAGE_SIZE as usize);
        Ok(())
    }

    #[test]
    fn test_short_virtual_pool_address_page_preserves_order_and_duplicates() -> anyhow::Result<()> {
        let mut addresses = vec!["first".to_string()];
        let mut page = 6;

        let is_last_page = append_virtual_pool_addresses_page(
            &mut addresses,
            &mut page,
            vec!["duplicate".to_string(), "duplicate".to_string()],
        )?;

        assert!(is_last_page);
        assert_eq!(page, 6);
        assert_eq!(addresses, vec!["first", "duplicate", "duplicate"]);
        Ok(())
    }

    #[test]
    fn test_virtual_pool_address_page_overflow_returns_error_without_appending() {
        let mut addresses = vec!["existing".to_string()];
        let mut page = u32::MAX;
        let page_addresses = vec!["new".to_string(); ALL_VIRTUAL_POOL_ADDRESSES_PAGE_SIZE as usize];

        let result = append_virtual_pool_addresses_page(&mut addresses, &mut page, page_addresses);

        assert!(result.is_err());
        assert_eq!(page, u32::MAX);
        assert_eq!(addresses, vec!["existing"]);
    }
}
