//! Raw DeDust API v4 pool-registry request variants.

use derive_more::From;

/// A request supported by the DeDust API v4 pool-registry client.
#[derive(Clone, From)]
#[non_exhaustive]
pub enum V4Request {
    /// Load every Classic (CPMM v1) pool descriptor.
    #[from(skip)]
    AllClassicPools,
    /// Load every Stable-swap pool descriptor.
    #[from(skip)]
    AllStablePools,
    /// Load every CPMM v2 pool descriptor and its fee configuration.
    #[from(skip)]
    AllCpmmPools,
    /// Load every Uranus launchpad pool descriptor.
    #[from(skip)]
    AllUranusPools,
}

impl From<&V4Request> for V4Request {
    fn from(request: &V4Request) -> Self { request.clone() }
}
