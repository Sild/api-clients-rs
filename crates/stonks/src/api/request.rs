//! Raw Stonks request variants and query parameters.
//!
//! Names and fields intentionally mirror the upstream wire contract.

use derive_more::From;
use derive_setters::Setters;
use serde::Serialize;

/// A request supported by the unversioned Stonks API client.
#[derive(Clone, From)]
#[non_exhaustive]
pub enum Request {
    /// Load public-token symbols, addresses, and raw tax percentages.
    #[from(skip)]
    PublicTokens,
    /// Load one page of raw TON addresses for discovered Virtual Pools.
    VirtualPoolAddresses(VirtualPoolAddressesParams),
    /// Load raw TON addresses from every Virtual Pool page sequentially.
    #[from(skip)]
    AllVirtualPoolAddresses,
}

/// Pagination parameters for one Virtual Pool address page.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct VirtualPoolAddressesParams {
    /// Zero-based page number.
    pub page: u32,
    /// Maximum number of addresses requested for the page.
    pub size: u32,
}

impl VirtualPoolAddressesParams {
    /// Create pagination parameters from a zero-based page number and page size.
    #[must_use]
    pub fn new(page: u32, size: u32) -> Self { Self { page, size } }
}

impl From<&Request> for Request {
    fn from(request: &Request) -> Self { request.clone() }
}
