use derive_more::From;
use derive_setters::Setters;
use serde_derive::Serialize;

#[derive(Clone, From)]
#[non_exhaustive]
pub enum V2Request {
    #[from(skip)]
    Assets,
    #[from(skip)]
    Pools,
    #[from(skip)]
    PoolsLite,
    #[from(skip)]
    PoolTrades(String),
    RoutingPlan(RoutingPlanParams),
}

/// Expecting addresses in format `workchain_id:hex_hash`
/// Check tests for examples
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct RoutingPlanParams {
    pub from: String,
    pub to: String,
    pub amount: String,
}

impl From<&V2Request> for V2Request {
    fn from(request: &V2Request) -> Self { request.clone() }
}

impl RoutingPlanParams {
    pub fn new(from: &str, to: &str, amount: &str) -> Self {
        let from = match from {
            "0:0000000000000000000000000000000000000000000000000000000000000000" => "native".to_string(),
            x => format!("jetton:{x}"),
        };
        let to = match to {
            "0:0000000000000000000000000000000000000000000000000000000000000000" => "native".to_string(),
            x => format!("jetton:{x}"),
        };
        Self {
            from,
            to,
            amount: amount.to_string(),
        }
    }
}
