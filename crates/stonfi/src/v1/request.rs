use derive_more::From;
use derive_setters::Setters;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, From)]
#[non_exhaustive]
pub enum V1Request {
    #[from(skip)]
    Assets,
    AssetsQuery(AssetsQueryParams),
    AssetsSearch(AssetsSearchParams),
    #[from(skip)]
    Asset(String),
    Farms(FarmsParams),
    #[from(skip)]
    Farm(String),
    FarmByPool(FarmByPoolParams),
    JettonWalletAddress(JettonWalletAddressParams),
    LiquidityProvisionSimulate(LiquidityProvisionSimulateParams),
    Markets(MarketsParams),
    PoolQuery(PoolQueryParams),
    Pools(PoolsParams),
    PoolsByMarket(PoolsByMarketParams),
    #[from(skip)]
    Pool(String),
    ReverseSwapSimulate(ReverseSwapSimulateParams),
    #[from(skip)]
    Router(String),
    Routers(RoutersParams),
    SwapSimulate(SwapSimulateParams),
    SwapStatus(SwapStatusParams),
    StatsDex(StatsDexParams),
    StatsFeeAccruals(StatsFeeAccrualsParams),
    StatsFeeWithdrawals(StatsFeeWithdrawalsParams),
    StatsFees(StatsFeesParams),
    StatsOperations(StatsOperationsParams),
    StatsPool(StatsPoolParams),
    #[from(skip)]
    StatsStaking,
    TransactionQuery(TransactionQueryParams),
    #[from(skip)]
    TransactionActionTree(String),
    WalletAsset(WalletAssetParams),
    WalletAssets(WalletAssetsParams),
    WalletFarm(WalletFarmParams),
    WalletFarms(WalletFarmsParams),
    WalletFeeVaults(WalletFeeVaultsParams),
    WalletOperations(WalletOperationsParams),
    WalletPool(WalletPoolParams),
    WalletPools(WalletPoolsParams),
    WalletStakes(WalletStakesParams),
    WalletTransactionsLast(WalletTransactionsLastParams),
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AssetsQueryParams {
    pub condition: Option<String>,
    pub unconditional_assets: Vec<String>,
    pub wallet_address: Option<String>,
}

impl AssetsQueryParams {
    pub fn new() -> Self { Self::default() }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AssetsSearchParams {
    pub search_string: String,
    pub condition: Option<String>,
    #[serde(rename = "unconditional_asset")]
    pub unconditional_assets: Option<Vec<String>>,
    pub limit: Option<u32>,
    pub wallet_address: Option<String>,
}

impl AssetsSearchParams {
    pub fn new(search_string: impl Into<String>) -> Self {
        Self {
            search_string: search_string.into(),
            condition: None,
            unconditional_assets: None,
            limit: None,
            wallet_address: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct SwapSimulateParams {
    pub offer_address: String,
    pub ask_address: String,
    pub units: String,
    pub slippage_tolerance: String,
    pub referral_address: Option<String>,
    pub referral_fee: Option<String>,
    pub dex_v2: Option<bool>,
}

impl SwapSimulateParams {
    pub fn new(
        offer_address: impl Into<String>,
        ask_address: impl Into<String>,
        units: impl Into<String>,
        slippage_tolerance: impl Into<String>,
    ) -> Self {
        Self {
            offer_address: offer_address.into(),
            ask_address: ask_address.into(),
            units: units.into(),
            slippage_tolerance: slippage_tolerance.into(),
            referral_address: None,
            referral_fee: None,
            dex_v2: Some(true),
        }
    }
}

impl Default for SwapSimulateParams {
    fn default() -> Self { Self::new("", "", "", "") }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct ReverseSwapSimulateParams {
    pub offer_address: String,
    pub ask_address: String,
    pub units: String,
    pub slippage_tolerance: String,
    pub pool_address: Option<String>,
    pub referral_address: Option<String>,
    pub referral_fee_bps: Option<String>,
    pub dex_v2: Option<bool>,
    pub dex_version: Option<Vec<String>>,
}

impl ReverseSwapSimulateParams {
    pub fn new(
        offer_address: impl Into<String>,
        ask_address: impl Into<String>,
        units: impl Into<String>,
        slippage_tolerance: impl Into<String>,
    ) -> Self {
        Self {
            offer_address: offer_address.into(),
            ask_address: ask_address.into(),
            units: units.into(),
            slippage_tolerance: slippage_tolerance.into(),
            pool_address: None,
            referral_address: None,
            referral_fee_bps: None,
            dex_v2: Some(true),
            dex_version: None,
        }
    }
}

impl Default for ReverseSwapSimulateParams {
    fn default() -> Self { Self::new("", "", "", "") }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct PoolsParams {
    pub dex_v2: bool,
}

impl Default for PoolsParams {
    fn default() -> Self { Self { dex_v2: true } }
}

impl PoolsParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct MarketsParams {
    pub dex_v2: bool,
}

impl Default for MarketsParams {
    fn default() -> Self { Self { dex_v2: true } }
}

impl MarketsParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct RoutersParams {
    pub dex_v2: bool,
}

impl Default for RoutersParams {
    fn default() -> Self { Self { dex_v2: true } }
}

impl RoutersParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct FarmByPoolParams {
    pub pool_address: String,
}

impl FarmByPoolParams {
    pub fn new(pool_address: impl Into<String>) -> Self {
        Self {
            pool_address: pool_address.into(),
        }
    }
}

#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct FarmsParams {
    pub dex_v2: bool,
}

impl Default for FarmsParams {
    fn default() -> Self { Self { dex_v2: true } }
}

impl FarmsParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct PoolsByMarketParams {
    pub asset0_address: String,
    pub asset1_address: String,
}

impl PoolsByMarketParams {
    pub fn new(asset0_address: impl Into<String>, asset1_address: impl Into<String>) -> Self {
        Self {
            asset0_address: asset0_address.into(),
            asset1_address: asset1_address.into(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolQueryParams {
    pub condition: Option<String>,
    pub dex_v2: Option<bool>,
    pub limit: Option<u32>,
    pub search_terms: Vec<String>,
    pub sort_by: Vec<String>,
    pub unconditional_assets: Vec<String>,
    pub wallet_address: Option<String>,
}

impl PoolQueryParams {
    pub fn new() -> Self {
        Self {
            dex_v2: Some(true),
            ..Self::default()
        }
    }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct SwapStatusParams {
    pub router_address: String,
    pub owner_address: String,
    pub query_id: String,
}

impl SwapStatusParams {
    pub fn new(
        router_address: impl Into<String>,
        owner_address: impl Into<String>,
        query_id: impl Into<String>,
    ) -> Self {
        Self {
            router_address: router_address.into(),
            owner_address: owner_address.into(),
            query_id: query_id.into(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsDexParams {
    pub since: Option<String>,
    pub until: Option<String>,
}

impl StatsDexParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct StatsFeeAccrualsParams {
    pub referrer_address: String,
    pub since: String,
    pub until: String,
}

impl StatsFeeAccrualsParams {
    pub fn new(referrer_address: impl Into<String>, since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            referrer_address: referrer_address.into(),
            since: since.into(),
            until: until.into(),
        }
    }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct StatsFeeWithdrawalsParams {
    pub referrer_address: String,
    pub since: String,
    pub until: String,
}

impl StatsFeeWithdrawalsParams {
    pub fn new(referrer_address: impl Into<String>, since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            referrer_address: referrer_address.into(),
            since: since.into(),
            until: until.into(),
        }
    }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct StatsFeesParams {
    pub referrer_address: String,
    pub since: String,
    pub until: String,
}

impl StatsFeesParams {
    pub fn new(referrer_address: impl Into<String>, since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            referrer_address: referrer_address.into(),
            since: since.into(),
            until: until.into(),
        }
    }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct JettonWalletAddressParams {
    pub asset_address: String,
    pub owner_address: String,
}

impl JettonWalletAddressParams {
    pub fn new(asset_address: impl Into<String>, owner_address: impl Into<String>) -> Self {
        Self {
            asset_address: asset_address.into(),
            owner_address: owner_address.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum LiquidityProvisionType {
    Initial,
    #[default]
    Balanced,
    Arbitrary,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct LiquidityProvisionSimulateParams {
    pub provision_type: LiquidityProvisionType,
    pub pool_address: Option<String>,
    pub wallet_address: Option<String>,
    pub token_a: String,
    pub token_b: String,
    pub token_a_units: Option<String>,
    pub token_b_units: Option<String>,
    pub slippage_tolerance: String,
}

impl LiquidityProvisionSimulateParams {
    pub fn new(
        provision_type: LiquidityProvisionType,
        token_a: impl Into<String>,
        token_b: impl Into<String>,
        slippage_tolerance: impl Into<String>,
    ) -> Self {
        Self {
            provision_type,
            pool_address: None,
            wallet_address: None,
            token_a: token_a.into(),
            token_b: token_b.into(),
            token_a_units: None,
            token_b_units: None,
            slippage_tolerance: slippage_tolerance.into(),
        }
    }
}

impl Default for LiquidityProvisionSimulateParams {
    fn default() -> Self { Self::new(LiquidityProvisionType::Balanced, "", "", "") }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsOperationsParams {
    pub since: String,
    pub until: String,
    pub pool_address: Option<String>,
}

impl StatsOperationsParams {
    pub fn new(since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            since: since.into(),
            until: until.into(),
            pool_address: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsPoolParams {
    pub since: String,
    pub until: String,
    pub pool_address: Option<String>,
}

impl StatsPoolParams {
    pub fn new(since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            since: since.into(),
            until: until.into(),
            pool_address: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TransactionQueryParams {
    pub wallet_address: Option<String>,
    pub query_id: Option<u64>,
    pub min_tx_timestamp: Option<String>,
    pub ext_msg_hash: Option<String>,
}

impl TransactionQueryParams {
    pub fn new() -> Self { Self::default() }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct WalletAssetsParams {
    pub wallet_address: String,
}

impl WalletAssetsParams {
    pub fn new(wallet_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
        }
    }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct WalletAssetParams {
    pub wallet_address: String,
    pub asset_address: String,
}

impl WalletAssetParams {
    pub fn new(wallet_address: impl Into<String>, asset_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
            asset_address: asset_address.into(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletPoolsParams {
    pub wallet_address: String,
    pub dex_v2: Option<bool>,
}

impl WalletPoolsParams {
    pub fn new(wallet_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
            dex_v2: Some(true),
        }
    }
}

impl Default for WalletPoolsParams {
    fn default() -> Self { Self::new("") }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct WalletPoolParams {
    pub wallet_address: String,
    pub pool_address: String,
}

impl WalletPoolParams {
    pub fn new(wallet_address: impl Into<String>, pool_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
            pool_address: pool_address.into(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletFarmsParams {
    pub wallet_address: String,
    pub dex_v2: Option<bool>,
    pub only_active: Option<bool>,
}

impl WalletFarmsParams {
    pub fn new(wallet_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
            dex_v2: Some(true),
            only_active: None,
        }
    }
}

impl Default for WalletFarmsParams {
    fn default() -> Self { Self::new("") }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct WalletFarmParams {
    pub wallet_address: String,
    pub farm_address: String,
}

impl WalletFarmParams {
    pub fn new(wallet_address: impl Into<String>, farm_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
            farm_address: farm_address.into(),
        }
    }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct WalletStakesParams {
    pub wallet_address: String,
}

impl WalletStakesParams {
    pub fn new(wallet_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletOperationsParams {
    pub wallet_address: String,
    pub since: String,
    pub until: String,
    pub op_type: Vec<String>,
    pub dex_v2: Option<bool>,
}

impl WalletOperationsParams {
    pub fn new(wallet_address: impl Into<String>, since: impl Into<String>, until: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
            since: since.into(),
            until: until.into(),
            op_type: Vec::new(),
            dex_v2: Some(true),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletTransactionsLastParams {
    pub wallet_address: String,
    pub limit: Option<u32>,
    pub min_tx_timestamp: Option<String>,
}

impl WalletTransactionsLastParams {
    pub fn new(wallet_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
            limit: None,
            min_tx_timestamp: None,
        }
    }
}

#[derive(Serialize, Clone, Default, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct WalletFeeVaultsParams {
    pub wallet_address: String,
}

impl WalletFeeVaultsParams {
    pub fn new(wallet_address: impl Into<String>) -> Self {
        Self {
            wallet_address: wallet_address.into(),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub(super) struct DexV2Query {
    pub(super) dex_v2: Option<bool>,
}

#[derive(Serialize)]
pub(super) struct JettonWalletAddressQuery<'a> {
    pub(super) owner_address: &'a str,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub(super) struct WalletFarmsQuery {
    pub(super) dex_v2: Option<bool>,
    pub(super) only_active: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub(super) struct WalletOperationsQuery<'a> {
    pub(super) since: &'a str,
    pub(super) until: &'a str,
    pub(super) op_type: &'a [String],
    pub(super) dex_v2: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub(super) struct WalletTransactionsLastQuery<'a> {
    pub(super) limit: Option<u32>,
    pub(super) min_tx_timestamp: Option<&'a str>,
}

impl From<&V1Request> for V1Request {
    fn from(request: &V1Request) -> Self { request.clone() }
}
