//! Raw Stonks request variants and query parameters.
//!
//! Names and fields intentionally mirror the upstream wire contract.
#![allow(missing_docs, reason = "raw request models mirror the upstream API contract")]

use derive_more::From;
use derive_setters::Setters;
use serde::Serialize;

#[derive(Clone, From)]
#[non_exhaustive]
pub enum Request {
    #[from(skip)]
    Assets,
    Pools(PoolsParams),
    #[from(skip)]
    AllPools,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct PoolsParams {
    pub page: usize,
    pub size: usize,
}

impl PoolsParams {
    pub fn new(page: usize, size: usize) -> Self { Self { page, size } }
}

impl From<&Request> for Request {
    fn from(request: &Request) -> Self { request.clone() }
}
