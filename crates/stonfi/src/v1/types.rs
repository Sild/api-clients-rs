use derive_setters::Setters;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Meta {
    pub custom_payload_api_uri: Option<String>,
    pub decimals: Option<i32>,
    pub display_name: Option<String>,
    pub image_url: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Asset {
    pub balance: Option<String>,
    pub blacklisted: bool,
    pub community: bool,
    pub contract_address: String,
    pub custom_payload_api_uri: Option<String>,
    pub decimals: u32,
    pub default_symbol: bool,
    pub deprecated: bool,
    pub dex_price_usd: Option<String>,
    pub dex_usd_price: Option<String>,
    pub display_name: Option<String>,
    pub extensions: Option<Vec<String>>,
    pub image_url: Option<String>,
    pub kind: String,
    pub popularity_index: Option<f64>,
    pub priority: u32,
    pub symbol: String,
    pub tags: Vec<String>,
    pub taxable: bool,
    pub third_party_price_usd: Option<String>,
    pub third_party_usd_price: Option<String>,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct QueryAsset {
    pub balance: Option<String>,
    pub contract_address: String,
    pub dex_price_usd: Option<String>,
    pub extensions: Option<Vec<String>>,
    pub kind: String,
    pub meta: Option<Meta>,
    pub pair_priority: Option<i64>,
    pub popularity_index: Option<f64>,
    pub tags: Vec<String>,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Pool {
    pub address: String,
    pub collected_token0_protocol_fee: String,
    pub collected_token1_protocol_fee: String,
    pub deprecated: bool,
    pub lp_fee: String,
    pub lp_price_usd: String,
    pub lp_total_supply: String,
    pub lp_total_supply_usd: String,
    pub protocol_fee: String,
    pub protocol_fee_address: String,
    pub reserve0: String,
    pub reserve1: String,
    pub router_address: String,
    pub token0_address: String,
    pub token1_address: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Router {
    pub address: String,
    pub major_version: i64,
    pub minor_version: i64,
    pub pool_creation_enabled: bool,
    pub pton_master_address: String,
    pub pton_version: String,
    pub pton_wallet_address: String,
    pub router_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FarmReward {
    pub address: Option<String>,
    pub remaining_rewards: String,
    pub reward_rate_24h: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FarmNftReward {
    pub address: String,
    pub amount: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FarmNft {
    pub address: String,
    pub create_timestamp: String,
    pub min_unstake_timestamp: String,
    pub nonclaimed_rewards: String,
    pub rewards: Vec<FarmNftReward>,
    pub staked_tokens: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct Farm {
    pub locked_total_lp: String,
    pub locked_total_lp_usd: String,
    pub min_stake_duration_s: String,
    pub minter_address: String,
    pub nft_infos: Vec<FarmNft>,
    pub pool_address: String,
    pub rewards: Vec<FarmReward>,
    pub status: String,
}

// Transaction action models.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TxId {
    pub lt: i64,
    pub hash: String,
    pub contract_address: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum Action {
    Amm(AmmAction),
    Farm(FarmAction),
    Other(OtherAction),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum AmmAction {
    Swap(ActionData<DexMajorVersion, SwapData, ActionStatus<SwapResult>>),
    ProvideLiquidity(
        WithEffects<
            ActionData<DexMajorVersion, ProvideLiquidityData, ActionStatus<ProvideLiquidityResult>>,
            ProvideLiquidityEffect,
        >,
    ),
    DirectAddLiquidity(ActionData<DexMajorVersion, DirectAddLiquidityData, ActionStatus<CbAddLiquidityResult>>),
    JettonBurn(ActionData<DexMajorVersion, BurnData, ActionStatus<BurnNotificationResult>>),
    CollectFees(ActionData<DexMajorVersion, CollectFeesData, ActionStatus<CollectFeesResult>>),
    WithdrawFee(ActionData<DexMajorVersion, WithdrawFeesData, ActionStatus<WithdrawFeeResult>>),
    RefundLiquidity(ActionData<DexMajorVersion, RefundLiquidityData, ActionStatus<RefundResult>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum FarmAction {
    FarmDeposit(ActionData<FarmMajorVersion, FarmDepositData, ActionStatus<FarmMinterStakeResult>>),
    FarmWithdraw(
        WithEffects<
            ActionData<FarmMajorVersion, FarmWithdrawData, ActionStatus<FarmNftUnstakeResult>>,
            FarmWithdrawEffect,
        >,
    ),
    FarmClaimRewards(
        WithEffects<
            ActionData<FarmMajorVersion, FarmClaimRewardsData, ActionStatus<FarmNftClaimRewardsResult>>,
            FarmClaimRewardsEffect,
        >,
    ),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum OtherAction {
    Other(ActionData<Blank, OtherTxPayload, UnknownActionStatus>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct ActionData<Version, Payload, Result> {
    pub contract_major_version: Option<Version>,
    pub destination: Option<String>,
    pub tx_payload: Payload,
    pub status: Result,
    pub prev_index: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WithEffects<T, Effect> {
    #[serde(flatten)]
    pub data: T,

    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "@type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ActionStatus<Result> {
    Completed(CompletedActionStatus<Result>),
    #[default]
    Pending,
    Aborted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct CompletedActionStatus<Result> {
    pub tx_id: TxId,
    pub success: bool,

    #[serde(flatten)]
    pub data: Result,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FarmMajorVersion {
    V1,
    V2,
    V3,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DexMajorVersion {
    V1,
    V2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct SwapData {
    // Common fields
    pub opcode: u32,
    pub query_id: u64,
    pub ask_jetton_wallet_address: String,
    pub min_ask_amount: String,
    pub referral_address: Option<String>,

    // V1-specific fields
    pub user_wallet_address: Option<String>,

    // V2-specific fields
    pub refund_address: Option<String>,
    pub excesses_address: Option<String>,
    pub deadline: Option<u64>,
    pub receiver_address: Option<String>,
    pub dex_forward_gas_amount: Option<String>,
    pub forward_gas_amount: Option<String>,

    pub referral_val: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum SwapResult {
    #[serde(rename = "swap_ok")]
    Ok {
        ask_jetton_wallet: String,
        ask_jetton: String,
        ask_jetton_amount: String,
        offer_jetton_wallet: String,
        offer_jetton: String,
        offer_jetton_amount: String,
        fwd_gas: Option<String>,
        is_reversed: bool,
    },
    #[serde(rename = "swap_ok_ref")]
    OkRef {
        ask_jetton_wallet: String,
        ask_jetton: Option<String>,
        ask_jetton_amount: String,
        offer_jetton_wallet: String,
        offer_jetton: Option<String>,
        offer_jetton_amount: String,
        ref_offer_jetton_wallet: String,
        ref_ask_jetton_wallet: String,
        ref_ask_amount: String,
        fwd_gas: Option<String>,
        is_reversed: bool,
    },
    #[serde(rename = "swap_refund_no_liq")]
    RefundNoLiquidity {
        ask_jetton_wallet: String,
        ask_amount: String,
    },
    #[serde(rename = "swap_refund_reserve_err")]
    RefundReserveError {
        ask_jetton_wallet: String,
        ask_amount: String,
    },
    #[serde(rename = "swap_refund_slippage")]
    RefundSlippageError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_pool_locked")]
    RefundPoolLockedError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_fee_out_of_bounds")]
    RefundSwapFeeOutOfBoundError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_tx_expired")]
    RefundTxExpiredError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_zero_output_err")]
    RefundZeroOutputError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_invalid_caller_err")]
    RefundInvalidCallerError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_insufficient_gas_err")]
    RefundInsufficientGasError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_wrong_address_err")]
    RefundWrongAddressError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_low_liquidity_err")]
    RefundLowLiquidityError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_math_err")]
    RefundMathError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_invalid_amount_err")]
    RefundInvalidAmountError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_invalid_call_err")]
    RefundInvalidCallError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_fee_out_range_err")]
    RefundFeeOutRangeError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_invalid_token_err")]
    RefundInvalidTokenError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_empty_not_allowed_err")]
    RefundEmptyNotAllowedError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_max_in_ratio_err")]
    RefundMaxInRatio {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_empty_cell_ratio_err")]
    RefundEmptyCellRatio {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_cell_underflow_err")]
    RefundCellUnderflowError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_wrong_workchain_err")]
    RefundWrongWorkchainError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_0_out")]
    RefundZeroOutError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_x_out_of_bounds")]
    XOutOfBoundsError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_y_out_of_bounds")]
    YOutOfBoundsError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_product_out_of_bounds")]
    ProductOfBoundsError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_invalid_exponent")]
    InvalidExponentError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_out_of_bounds")]
    OutOfBoundsError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_zero_base")]
    ZeroBaseError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_negative_power")]
    NegativePowerError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_zero_division")]
    ZeroDivisionError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_invalid_input")]
    InvalidInputError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_cmplement_out_of_range")]
    CmplmentOutOfRangeError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_not_converge_range")]
    NotConvergeError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_fd_zero")]
    DfZeroError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "bounced")]
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct ProvideLiquidityData {
    // Common fields
    pub opcode: u32,
    pub query_id: u64,
    pub router_wallet_address: String,
    pub min_lp_out: String,
    // V2-specific fields
    pub refund_address: Option<String>,
    pub excesses_address: Option<String>,
    pub deadline: Option<u64>,
    pub receiver_address: Option<String>,
    pub both_positive: Option<u8>,
    pub dex_forward_gas_amount: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum ProvideLiquidityResult {
    #[serde(rename = "provide_liquidity_ok")]
    AddLiquidity {
        amount0_out: String,
        amount1_out: String,
        min_lp_out: String,
        fwd_amount: Option<String>,
        /// Owner of new liquidity tokens
        to_address: Option<String>,
        refund_address: Option<String>,
        excess_address: Option<String>,
    },
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "@type", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum ProvideLiquidityEffect {
    #[default]
    Never,
    CbAddLiquidity(EffectData<CbAddLiquidityResult>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum CbAddLiquidityResult {
    #[serde(rename = "add_liquidity_ok")]
    AddLiquidity {
        /// The amount of 0th asset provided
        amount0_out: String,
        /// The amount of 1st asset provided
        amount1_out: String,
        /// Minimal LP token amount to provide
        min_lp_out: String,
        /// Optional destination address
        to_address: Option<String>,
        /// Optional forwarded amount
        fwd_amount: Option<String>,
        /// Optional refund address
        refund_address: Option<String>,
        /// Optional excess address
        excess_address: Option<String>,
    },
    #[serde(rename = "mint_lp_ok")]
    MintLp {
        lp_out: String,
    },
    Empty,
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct EffectData<Result> {
    pub status: ActionStatus<Result>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum UnknownActionStatus {
    Completed {},
    Pending,
    Aborted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DirectAddLiquidityData {
    pub opcode: u32,
    pub query_id: u64,
    pub amount_0: String,
    pub amount_1: String,
    pub min_lp_to_mint: String,

    // V2-specific fields
    pub dex_forward_gas_amount: Option<String>,
    pub user_wallet_address: Option<String>,
    pub refund_address: Option<String>,
    pub excesses_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct BurnData {
    pub opcode: u32,
    pub query_id: u64,
    pub amount: String,
    pub response_destination: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum BurnNotificationResult {
    #[serde(rename = "burn_ok")]
    Ok {
        from_address: String,
        amount0_out: String,
        amount1_out: String,
    },
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct CollectFeesData {
    pub opcode: u32,
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WithdrawFeesData {
    pub opcode: u32,
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct RefundLiquidityData {
    pub opcode: u32,
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FarmDepositData {
    pub opcode: u32,
    pub query_id: u64,
    pub owner_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FarmWithdrawData {
    pub opcode: u32,
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FarmClaimRewardsData {
    pub opcode: u32,
    pub query_id: u64,
    pub claim_all: Option<bool>,
    pub pool_index: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum CollectFeesResult {
    #[serde(rename = "collect_fees_ok")]
    Ok {
        protocol_fee_address: String,
        collected_token0_protocol_fee: String,
        collected_token1_protocol_fee: String,
    },

    #[serde(rename = "collect_fees_ok_with_rewards")]
    OkWithReward {
        protocol_fee_address: String,
        collected_token0_protocol_fee: String,
        collected_token1_protocol_fee: String,
        sender_address: String,
        reward0: String,
        reward1: String,
    },

    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum WithdrawFeeResult {
    Ok {
        amount_out: String,
        token_address: String,
        to_address: String,
    },
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum RefundResult {
    #[serde(rename = "refund_ok")]
    Ok {
        user_address: String,
        amount0_out: String,
        router_jetton0_address: String,
        amount1_out: String,
        router_jetton1_address: String,
    },
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum FarmMinterStakeResult {
    #[serde(rename = "farm_minter_stake_ok")]
    Ok {},
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum FarmNftClaimRewardsResult {
    #[serde(rename = "farm_nft_claim_rewards_ok")]
    Ok {},
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum FarmNftUnstakeResult {
    #[serde(rename = "farm_nft_unstake_ok")]
    Ok {},
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "@type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum FarmWithdrawEffect {
    #[default]
    Never,
    JettonTransfer(EffectData<JettonTransferResult>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "@type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum FarmClaimRewardsEffect {
    #[default]
    Never,
    JettonTransfer(EffectData<JettonTransferResult>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[non_exhaustive]
pub enum JettonTransferResult {
    Ok(JettonTransferData),
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct JettonTransferData {
    pub opcode: u32,
    pub query_id: u64,
    pub amount: String,
    pub destination: String,
    pub response_destination: String,
    pub forward_ton_amount: String,
    pub jetton_master: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OtherTxPayload {
    #[serde(rename = "unknown")]
    Invalid { opcode: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct Blank;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct TransactionActionTree {
    pub tx_chain_completed: bool,
    pub initial_tx_id: TxId,
    pub actions: Vec<Action>,
}

// Swap status models.
#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct SwapStatus {
    pub address: String,
    pub balance_deltas: SwapStatusBalanceDeltas,
    pub coins: String,
    pub exit_code: String,
    pub logical_time: String,
    pub query_id: String,
    pub tx_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SwapStatusBalanceDeltas {
    Map(BTreeMap<String, String>),
    Text(String),
}

impl Default for SwapStatusBalanceDeltas {
    fn default() -> Self { Self::Map(BTreeMap::new()) }
}

// Stats models.
#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct DexStats {
    pub trades: u64,
    pub tvl: String,
    pub unique_wallets: u64,
    pub volume_usd: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FeeAccrual {
    pub pool_address: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct FeeWithdrawal {
    pub vault_address: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct AssetFeeStats {
    pub accrued: String,
    pub accrued_usd: Option<String>,
    pub asset_address: String,
    pub withdrawn: String,
    pub withdrawn_usd: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsOperationInfo {
    pub operation: StatsOperation,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct StatsOperation {
    pub pool_address: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct PoolStats {
    pub pool_address: String,
}

// Wallet models.
#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletFeeVault {
    pub asset_address: String,
    pub balance: String,
    pub router_address: String,
    pub vault_address: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletOperationInfo {
    pub asset0_info: Asset,
    pub asset1_info: Asset,
    pub operation: WalletOperation,
}

#[derive(Deserialize, Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletOperation {
    pub asset0_address: String,
    pub asset0_amount: String,
    pub asset0_delta: String,
    pub asset0_reserve: String,
    pub asset1_address: String,
    pub asset1_amount: String,
    pub asset1_delta: String,
    pub asset1_reserve: String,
    pub destination_wallet_address: String,
    pub exit_code: String,
    pub fee_asset_address: Option<String>,
    pub lp_fee_amount: String,
    pub lp_token_delta: String,
    pub lp_token_supply: String,
    pub operation_type: String,
    pub pool_address: String,
    pub pool_tx_hash: String,
    pub pool_tx_lt: i64,
    pub pool_tx_timestamp: String,
    pub protocol_fee_amount: String,
    pub referral_address: Option<String>,
    pub referral_fee_amount: String,
    pub router_address: String,
    pub success: bool,
    pub wallet_address: String,
    pub wallet_tx_hash: String,
    pub wallet_tx_lt: String,
    pub wallet_tx_timestamp: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletStakeNft {
    pub address: String,
    pub image_url: String,
    pub min_unstaking_timestamp: String,
    pub minted_gemston: String,
    pub staked_tokens: String,
    pub staking_timestamp: String,
    pub status: String,
    pub unstake_timestamp: Option<String>,
    pub voting_power: String,
}

#[derive(Deserialize, Debug, Clone, Default, Setters, PartialEq, Eq)]
#[setters(prefix = "with_", strip_option)]
#[non_exhaustive]
pub struct WalletTransactionId {
    pub contract_address: Option<String>,
    pub hash: String,
    pub lt: i64,
}
