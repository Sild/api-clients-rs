use crate::v1::request::LiquidityProvisionType;
use crate::v1::types::{
    Asset, AssetFeeStats, DexStats, Farm, FeeAccrual, FeeWithdrawal, Pool, PoolStats, QueryAsset, Router,
    StatsOperationInfo, SwapStatus, TransactionActionTree, TxId, WalletFeeVault, WalletOperationInfo, WalletStakeNft,
    WalletTransactionId,
};
use derive_more::From;
use derive_setters::Setters;
use serde_derive::Deserialize;

#[macro_export]
macro_rules! unwrap_response {
    ($variant:ident, $result:expr) => {
        match $result {
            $crate::v1::V1Response::$variant(inner) => Ok(inner),
            other => Err($crate::api_clients_core::ApiClientsError::UnexpectedResponse(format!(
                "ApiClientError: expected {}, but got {:?}",
                stringify!($variant),
                other
            ))),
        }
    };
}

#[derive(Deserialize, Debug, Clone, From)]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum V1Response {
    Assets(AssetsResponse),
    AssetsQuery(AssetsQueryResponse),
    AssetsSearch(AssetsSearchResponse),
    Asset(AssetResponse),
    Farms(FarmsResponse),
    Farm(FarmResponse),
    #[from(skip)]
    FarmByPool(FarmsResponse),
    JettonWalletAddress(AddressResponse),
    LiquidityProvisionSimulate(LiquidityProvisionSimulateResponse),
    Markets(MarketsResponse),
    Pools(PoolsResponse),
    #[from(skip)]
    PoolQuery(PoolsResponse),
    Pool(PoolResponse),
    Routers(RoutersResponse),
    Router(RouterResponse),
    SwapSimulate(SwapSimulateResponse),
    #[from(skip)]
    ReverseSwapSimulate(SwapSimulateResponse),
    SwapStatus(SwapStatusResponse),
    StatsDex(StatsDexResponse),
    StatsFeeAccruals(StatsFeeAccrualsResponse),
    StatsFeeWithdrawals(StatsFeeWithdrawalsResponse),
    StatsFees(StatsFeesResponse),
    StatsOperations(StatsOperationsResponse),
    StatsPool(StatsPoolResponse),
    StatsStaking(StatsStakingResponse),
    TransactionActionTree(TransactionActionTreeResponse),
    TransactionQuery(TransactionQueryResponse),
    #[from(skip)]
    WalletAsset(AssetResponse),
    #[from(skip)]
    WalletAssets(AssetsResponse),
    #[from(skip)]
    WalletFarm(FarmResponse),
    #[from(skip)]
    WalletFarms(FarmsResponse),
    WalletFeeVaults(WalletFeeVaultsResponse),
    WalletOperations(WalletOperationsResponse),
    #[from(skip)]
    WalletPool(PoolResponse),
    #[from(skip)]
    WalletPools(PoolsResponse),
    WalletStakes(WalletStakesResponse),
    WalletTransactionsLast(WalletTransactionsLastResponse),
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AssetsResponse {
    pub asset_list: Vec<Asset>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AssetsQueryResponse {
    pub asset_list: Vec<QueryAsset>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AssetsSearchResponse {
    pub asset_list: Vec<QueryAsset>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AssetResponse {
    pub asset: Asset,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FarmsResponse {
    pub farm_list: Vec<Farm>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FarmResponse {
    pub farm: Farm,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AddressResponse {
    pub address: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct MarketsResponse {
    pub pairs: Vec<[String; 2]>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolsResponse {
    pub pool_list: Vec<Pool>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolResponse {
    pub pool: Pool,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct RoutersResponse {
    pub router_list: Vec<Router>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct RouterResponse {
    pub router: Router,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct SwapSimulateResponse {
    pub ask_address: String,
    pub ask_jetton_wallet: String,
    pub ask_units: String,
    pub fee_address: String,
    pub fee_percent: String,
    pub fee_units: String,
    pub min_ask_units: String,
    pub offer_address: String,
    pub offer_jetton_wallet: String,
    pub offer_units: String,
    pub pool_address: String,
    pub price_impact: String,
    pub router_address: String,
    pub slippage_tolerance: String,
    pub swap_rate: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "@type")]
#[non_exhaustive]
pub enum SwapStatusResponse {
    Found(SwapStatus),
    NotFound,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsDexResponse {
    pub since: String,
    pub until: String,
    pub stats: DexStats,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsFeeAccrualsResponse {
    pub operations: Vec<FeeAccrual>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsFeeWithdrawalsResponse {
    pub withdrawals: Vec<FeeWithdrawal>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsFeesResponse {
    pub assets_fee_stats: Vec<AssetFeeStats>,
    pub since: String,
    pub until: String,
    pub total_accrued_usd: Option<String>,
    pub total_withdrawn_usd: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsOperationsResponse {
    pub operations: Vec<StatsOperationInfo>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsPoolResponse {
    pub since: String,
    pub until: String,
    pub unique_wallets_count: u64,
    pub stats: Vec<PoolStats>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsStakingResponse {
    pub gemston_total_supply: String,
    pub ston_price_usd: String,
    pub ston_total_supply: String,
    pub total_staked_ston: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TransactionQueryResponse {
    pub tx_id: Option<TxId>,
    pub wallet_seqno: Option<u32>,
}

pub type TransactionActionTreeResponse = TransactionActionTree;

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct LiquidityProvisionSimulateResponse {
    pub estimated_lp_units: String,
    pub estimated_token_a_rate: String,
    pub estimated_token_a_units: String,
    pub estimated_token_b_rate: String,
    pub estimated_token_b_units: String,
    pub lp_account_address: Option<String>,
    pub lp_account_token_a_balance: Option<String>,
    pub lp_account_token_b_balance: Option<String>,
    pub lp_total_supply: String,
    pub min_lp_units: String,
    pub min_token_a_units: String,
    pub min_token_b_units: String,
    pub pool_address: String,
    pub price_impact: String,
    pub provision_type: LiquidityProvisionType,
    pub router: Router,
    pub router_address: String,
    pub token_a: String,
    pub token_a_units: String,
    pub token_b: String,
    pub token_b_units: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletFeeVaultsResponse {
    pub vault_list: Vec<WalletFeeVault>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletOperationsResponse {
    pub operations: Vec<WalletOperationInfo>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletStakesResponse {
    pub minted_gemston: String,
    pub nfts: Option<Vec<WalletStakeNft>>,
    pub staked_ston: String,
    pub ston_balance: String,
    pub voting_power: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletTransactionsLastResponse {
    pub seqno: u32,
    pub tx_list: Vec<WalletTransactionId>,
}
