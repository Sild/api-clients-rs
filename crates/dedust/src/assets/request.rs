//! Raw DeDust asset-registry request variants.

use derive_more::From;

/// A request supported by the DeDust asset-registry client.
#[derive(Clone, From)]
#[non_exhaustive]
pub enum AssetsRequest {
    /// Load the complete DeDust asset registry.
    #[from(skip)]
    List,
}

impl From<&AssetsRequest> for AssetsRequest {
    fn from(request: &AssetsRequest) -> Self { request.clone() }
}
