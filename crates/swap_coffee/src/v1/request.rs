//! Raw Swap Coffee API v1 request variants and query parameters.
//!
//! Names and fields intentionally mirror the upstream wire contract.
#![allow(missing_docs, reason = "raw request models mirror the upstream API contract")]

use derive_more::From;
use derive_setters::Setters;
use serde_derive::Serialize;

#[derive(Clone, From)]
#[non_exhaustive]
pub enum V1Request {
    #[from(skip)]
    Assets,
    Pools(Dexes),
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct Dexes {
    pub dexes: String,
}

impl Dexes {
    pub fn new(dexes: impl Into<String>) -> Self { Self { dexes: dexes.into() } }
}

impl From<&V1Request> for V1Request {
    fn from(request: &V1Request) -> Self { request.clone() }
}
