use anyhow::Result;
use api_clients_core::Executor;
use std::sync::Arc;
use std::time::Duration;
use stonfi_api_client::api_client::StonfiApiClient;
use stonfi_api_client::api_client::DEFAULT_API_V1_URL;
use stonfi_api_client::unwrap_response;
use stonfi_api_client::v1::*;

fn init_env() -> Result<StonfiApiClient> {
    let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).try_init();
    let executor = Executor::builder(DEFAULT_API_V1_URL).with_timeout(Duration::from_secs(60)).build()?;
    Ok(StonfiApiClient::builder().with_executor(Arc::new(executor)).build()?)
}

const WALLET_ADDRESS_WITH_POSITIONS: &str = "EQCariTouRoqxeqB9rVsXlSK5ML1XQW-QNxzzj4VXiQl9Vix";

#[tokio::test]
async fn test_swap_simulate_new_defaults_to_dex_v2() -> Result<()> {
    let params = SwapSimulateParams::new(
        "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo",
        "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs",
        "40000000",
        "0.001",
    );
    assert_eq!(params.dex_v2, Some(true));
    Ok(())
}

#[tokio::test]
#[ignore = "assets returns the full asset catalog; run explicitly when checking this heavy endpoint"]
async fn test_assets() -> Result<()> {
    let client = init_env()?;
    let request = V1Request::Assets;
    let response = unwrap_response!(Assets, client.v1.exec(&request).await?)?;
    assert_ne!(response.asset_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_assets_query() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        AssetsQuery,
        client
            .v1
            .exec(
                AssetsQueryParams::new()
                    .with_condition("!asset:liquidity:no & !asset:blacklisted".to_string())
                    .with_unconditional_assets(vec![]),
            )
            .await?
    )?;
    assert!(response.asset_list.len() > 1);

    let response = unwrap_response!(
        AssetsQuery,
        client
            .v1
            .exec(AssetsQueryParams::new().with_unconditional_assets(vec![
                "EQBfkgKxD8zkHquKL6pqZWGiQCkrbgXIw4ToqNRb9-RW0ba1".to_string(),
                "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs".to_string(),
            ]))
            .await?
    )?;
    assert_eq!(response.asset_list.len(), 3);
    Ok(())
}

#[tokio::test]
async fn test_assets_search() -> Result<()> {
    let client = init_env()?;
    let response =
        unwrap_response!(AssetsSearch, client.v1.exec(AssetsSearchParams::new("USDT").with_limit(1)).await?)?;
    assert!(response.asset_list.len() <= 1);
    Ok(())
}

#[tokio::test]
async fn test_asset() -> Result<()> {
    let client = init_env()?;
    let ton_addr = "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c";
    let request = V1Request::Asset(ton_addr.to_string());
    let response = unwrap_response!(Asset, client.v1.exec(&request).await?)?;
    assert_eq!(response.asset.contract_address, ton_addr);
    assert_eq!(response.asset.display_name, Some("Gram".to_string()));
    assert_eq!(response.asset.symbol, "GRAM");
    Ok(())
}

#[tokio::test]
async fn test_farms() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(Farms, client.v1.exec(FarmsParams::default()).await?)?;
    assert_ne!(response.farm_list, vec![]);

    Ok(())
}

#[tokio::test]
async fn test_farm() -> Result<()> {
    let client = init_env()?;
    let farm_minter = "EQCuq6v9szR4MrtVYN_kGHh2WMKU2ahJQzCL4J1VwfL1LSJm".to_string();
    let request = V1Request::Farm(farm_minter.clone());
    let response = unwrap_response!(Farm, client.v1.exec(&request).await?)?;
    assert_eq!(response.farm.minter_address, farm_minter);
    Ok(())
}

#[tokio::test]
async fn test_farm_by_pool() -> Result<()> {
    let client = init_env()?;
    let hmstr_usdt_pool = "EQBXg9I5MBvwv7O8Xd0ZOC6z7T6yoCojaBXQXoAYx6paDO2s";
    let response = unwrap_response!(FarmByPool, client.v1.exec(FarmByPoolParams::new(hmstr_usdt_pool)).await?)?;
    assert!(response.farm_list.iter().all(|farm| farm.pool_address == hmstr_usdt_pool));
    Ok(())
}

#[tokio::test]
async fn test_markets() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(Markets, client.v1.exec(MarketsParams::default()).await?)?;
    if let Some(pair) = response.pairs.first() {
        assert_eq!(pair.len(), 2);
    }
    Ok(())
}

#[tokio::test]
#[ignore = "pools returns the full pool catalog; run explicitly when checking this heavy endpoint"]
async fn test_pools() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(Pools, client.v1.exec(PoolsParams::default().with_dex_v2(false)).await?)?;
    assert_ne!(response.pool_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pools_by_market() -> Result<()> {
    let client = init_env()?;
    let params = PoolsByMarketParams::new(
        "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c", // ton
        "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs", // usdt
    );
    let response = unwrap_response!(Pools, client.v1.exec(params).await?)?;
    assert_ne!(response.pool_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_pool_query() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        PoolQuery,
        client.v1.exec(PoolQueryParams::new().with_search_terms(vec!["USDT".to_string()]).with_limit(1)).await?
    )?;
    assert!(response.pool_list.len() <= 1);
    Ok(())
}

#[tokio::test]
async fn test_pool() -> Result<()> {
    let client = init_env()?;
    let hmstr_usdt_pool = "EQBXg9I5MBvwv7O8Xd0ZOC6z7T6yoCojaBXQXoAYx6paDO2s";
    let request = V1Request::Pool(hmstr_usdt_pool.to_string());
    let response = unwrap_response!(Pool, client.v1.exec(&request).await?)?;
    assert_eq!(response.pool.address, hmstr_usdt_pool);
    assert_eq!(response.pool.token0_address, "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo"); // hmstr
    assert_eq!(response.pool.token1_address, "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs"); // usdt
    Ok(())
}

#[tokio::test]
async fn test_routers() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(Routers, client.v1.exec(RoutersParams::default()).await?)?;
    assert_ne!(response.router_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_router() -> Result<()> {
    let client = init_env()?;
    let addr = "EQB3ncyBUTjZUA5EnFKR5_EnOMI9V1tTEAAPaiU71gc4TiUt";
    let request = V1Request::Router(addr.to_string());
    let response = unwrap_response!(Router, client.v1.exec(&request).await?)?;
    assert_eq!(response.router.address, addr);
    Ok(())
}

#[tokio::test]
async fn test_swap_simulate() -> Result<()> {
    let client = init_env()?;

    let params = SwapSimulateParams::new(
        "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo", // hmstr
        "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs", // usdt
        "40000000",
        "0.001",
    );
    let response = unwrap_response!(SwapSimulate, client.v1.exec(params.clone()).await?)?;
    assert_eq!(response.offer_address, params.offer_address);
    assert_eq!(response.ask_address, params.ask_address);
    Ok(())
}

#[tokio::test]
async fn test_liquidity_provision_simulate() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        LiquidityProvisionSimulate,
        client
            .v1
            .exec(
                LiquidityProvisionSimulateParams::new(
                    LiquidityProvisionType::Balanced,
                    "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c",
                    "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs",
                    "0.001",
                )
                .with_pool_address("EQCGScrZe1xbyWqWDvdI6mzP-GAcAWFv6ZXuaJOuSqemxku4".to_string())
                .with_wallet_address(WALLET_ADDRESS_WITH_POSITIONS.to_string())
                .with_token_a_units("1000000000".to_string()),
            )
            .await?
    )?;

    assert_eq!(response.provision_type, LiquidityProvisionType::Balanced);
    assert_eq!(response.token_a, "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c");
    assert_eq!(response.token_b, "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs");
    Ok(())
}

#[tokio::test]
async fn test_jetton_wallet_address() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        JettonWalletAddress,
        client
            .v1
            .exec(JettonWalletAddressParams::new(
                "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs",
                WALLET_ADDRESS_WITH_POSITIONS,
            ))
            .await?
    )?;

    assert!(!response.address.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_reverse_swap_simulate() -> Result<()> {
    let client = init_env()?;
    let params = ReverseSwapSimulateParams::new(
        "EQAJ8uWd7EBqsmpSWaRdf_I-8R8-XHwh3gsNKhy-UrdrPcUo",
        "EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs",
        "1000000",
        "0.001",
    );
    let response = unwrap_response!(ReverseSwapSimulate, client.v1.exec(params.clone()).await?)?;
    assert_eq!(response.offer_address, params.offer_address);
    assert_eq!(response.ask_address, params.ask_address);
    Ok(())
}

#[tokio::test]
async fn test_tx_query() -> Result<()> {
    let client = init_env()?;
    let ext_msg_hash = "c88578dba0219474bc9eced97a5ed3c85d1bf1d48d4484832eb72113e982d181";
    let response = unwrap_response!(
        TransactionQuery,
        client
            .v1
            .exec(
                TransactionQueryParams::new()
                    .with_ext_msg_hash(ext_msg_hash.to_string())
                    .with_min_tx_timestamp("2025-11-25T00:00:00".to_string()),
            )
            .await?
    )?;

    assert!(response.tx_id.is_none());
    assert!(response.wallet_seqno.is_none());

    Ok(())
}

#[tokio::test]
async fn test_swap_status() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        SwapStatus,
        client
            .v1
            .exec(SwapStatusParams::new(
                "EQB3ncyBUTjZUA5EnFKR5_EnOMI9V1tTEAAPaiU71gc4TiUt",
                "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c",
                "0",
            ))
            .await?
    )?;
    match response {
        SwapStatusResponse::Found(status) => assert_eq!(status.query_id, "0"),
        SwapStatusResponse::NotFound => {}
        _ => anyhow::bail!("unexpected swap status response"),
    }
    Ok(())
}

#[tokio::test]
async fn test_stats_dex() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        StatsDex,
        client
            .v1
            .exec(
                StatsDexParams::new()
                    .with_since("2026-06-01T00:00:00".to_string())
                    .with_until("2026-06-02T00:00:00".to_string()),
            )
            .await?
    )?;
    assert!(!response.since.is_empty());
    assert!(!response.until.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_stats_fee_accruals() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        StatsFeeAccruals,
        client
            .v1
            .exec(StatsFeeAccrualsParams::new(
                "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c",
                "2026-06-01T00:00:00",
                "2026-06-02T00:00:00",
            ))
            .await?
    )?;
    assert!(response.operations.iter().all(|operation| !operation.pool_address.is_empty()));
    Ok(())
}

#[tokio::test]
async fn test_stats_fee_withdrawals() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        StatsFeeWithdrawals,
        client
            .v1
            .exec(StatsFeeWithdrawalsParams::new(
                "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c",
                "2026-06-01T00:00:00",
                "2026-06-02T00:00:00",
            ))
            .await?
    )?;
    assert!(response.withdrawals.iter().all(|withdrawal| !withdrawal.vault_address.is_empty()));
    Ok(())
}

#[tokio::test]
async fn test_stats_fees() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        StatsFees,
        client
            .v1
            .exec(StatsFeesParams::new(
                "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c",
                "2026-06-01T00:00:00",
                "2026-06-02T00:00:00",
            ))
            .await?
    )?;
    assert!(!response.since.is_empty());
    assert!(!response.until.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_stats_operations() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        StatsOperations,
        client
            .v1
            .exec(
                StatsOperationsParams::new("2026-06-01T00:00:00", "2026-06-02T00:00:00")
                    .with_pool_address("EQBXg9I5MBvwv7O8Xd0ZOC6z7T6yoCojaBXQXoAYx6paDO2s".to_string()),
            )
            .await?
    )?;
    assert!(response.operations.iter().all(|operation| !operation.operation.pool_address.is_empty()));
    Ok(())
}

#[tokio::test]
async fn test_stats_pool() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        StatsPool,
        client.v1.exec(StatsPoolParams::new("2026-06-01T00:00:00", "2026-06-02T00:00:00")).await?
    )?;
    assert!(!response.since.is_empty());
    assert!(!response.until.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_stats_staking() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(StatsStaking, client.v1.exec(V1Request::StatsStaking).await?)?;
    assert!(!response.total_staked_ston.is_empty());
    Ok(())
}

#[tokio::test]
#[ignore = "wallet assets returns the full asset catalog; run explicitly when checking this heavy endpoint"]
async fn test_wallet_assets() -> Result<()> {
    let client = init_env()?;
    let response =
        unwrap_response!(WalletAssets, client.v1.exec(WalletAssetsParams::new(WALLET_ADDRESS_WITH_POSITIONS)).await?)?;
    assert_ne!(response.asset_list, vec![]);
    assert!(response
        .asset_list
        .iter()
        .any(|asset| asset.wallet_address.as_deref() == Some(WALLET_ADDRESS_WITH_POSITIONS)));
    Ok(())
}

#[tokio::test]
async fn test_wallet_asset() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        WalletAsset,
        client
            .v1
            .exec(WalletAssetParams::new(
                WALLET_ADDRESS_WITH_POSITIONS,
                "EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c",
            ))
            .await?
    )?;
    assert_eq!(response.asset.wallet_address.as_deref(), Some(WALLET_ADDRESS_WITH_POSITIONS));
    Ok(())
}

#[tokio::test]
async fn test_wallet_pools() -> Result<()> {
    let client = init_env()?;
    let response =
        unwrap_response!(WalletPools, client.v1.exec(WalletPoolsParams::new(WALLET_ADDRESS_WITH_POSITIONS)).await?)?;
    assert_ne!(response.pool_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_wallet_pool() -> Result<()> {
    let client = init_env()?;
    let pool_address = "EQCGScrZe1xbyWqWDvdI6mzP-GAcAWFv6ZXuaJOuSqemxku4";
    let response = unwrap_response!(
        WalletPool,
        client.v1.exec(WalletPoolParams::new(WALLET_ADDRESS_WITH_POSITIONS, pool_address)).await?
    )?;
    assert_eq!(response.pool.address, pool_address);
    Ok(())
}

#[tokio::test]
async fn test_wallet_farms() -> Result<()> {
    let client = init_env()?;
    let response =
        unwrap_response!(WalletFarms, client.v1.exec(WalletFarmsParams::new(WALLET_ADDRESS_WITH_POSITIONS)).await?)?;
    assert_ne!(response.farm_list, vec![]);
    Ok(())
}

#[tokio::test]
async fn test_wallet_farm() -> Result<()> {
    let client = init_env()?;
    let farm_address = "EQD0eKCNL6SRy9ZOc8i6R7nvuIAcRs3BwREqq4Z4UzdXvnBz";
    let response = unwrap_response!(
        WalletFarm,
        client.v1.exec(WalletFarmParams::new(WALLET_ADDRESS_WITH_POSITIONS, farm_address)).await?
    )?;
    assert_eq!(response.farm.minter_address, farm_address);
    Ok(())
}

#[tokio::test]
async fn test_wallet_stakes() -> Result<()> {
    let client = init_env()?;
    let response =
        unwrap_response!(WalletStakes, client.v1.exec(WalletStakesParams::new(WALLET_ADDRESS_WITH_POSITIONS)).await?)?;
    assert!(!response.ston_balance.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_wallet_operations() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        WalletOperations,
        client
            .v1
            .exec(WalletOperationsParams::new(
                WALLET_ADDRESS_WITH_POSITIONS,
                "2026-06-01T00:00:00",
                "2026-06-02T00:00:00",
            ))
            .await?
    )?;
    assert!(response.operations.iter().all(|operation| !operation.operation.pool_address.is_empty()));
    Ok(())
}

#[tokio::test]
async fn test_wallet_transactions_last() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        WalletTransactionsLast,
        client.v1.exec(WalletTransactionsLastParams::new(WALLET_ADDRESS_WITH_POSITIONS).with_limit(1)).await?
    )?;
    assert!(response.tx_list.len() <= 1);
    Ok(())
}

#[tokio::test]
async fn test_wallet_fee_vaults() -> Result<()> {
    let client = init_env()?;
    let response = unwrap_response!(
        WalletFeeVaults,
        client.v1.exec(WalletFeeVaultsParams::new(WALLET_ADDRESS_WITH_POSITIONS)).await?
    )?;
    assert!(response.vault_list.iter().all(|vault| !vault.vault_address.is_empty()));
    Ok(())
}

#[tokio::test]
async fn test_tx_action_tree_swap() -> Result<()> {
    let client = init_env()?;
    let hash = "0a3154df02213c26a88714c7f75b70701d8230f1ed0acf0d96fcea7458a723fc";
    let request = V1Request::TransactionActionTree(hash.to_string());
    let response = unwrap_response!(TransactionActionTree, client.v1.exec(&request).await?)?;

    assert_eq!(response.initial_tx_id.hash, hash);

    let swap_ask_jetton_amount = response.actions.first().and_then(|action| match action {
        Action::Amm(AmmAction::Swap(data)) => match &data.status {
            ActionStatus::Completed(payload) => match &payload.data {
                SwapResult::Ok { ask_jetton_amount, .. } => Some(ask_jetton_amount.clone()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    });

    assert_eq!(swap_ask_jetton_amount, Some("23810546".to_string()));

    Ok(())
}

#[tokio::test]
async fn test_tx_action_tree_swap_plus_lp() -> Result<()> {
    let client = init_env()?;
    let hash = "ccd5d1c148ffc9157dd248b8350da05a17e958d913150fde3904e85e82c5b5a9";
    let request = V1Request::TransactionActionTree(hash.to_string());
    let response = unwrap_response!(TransactionActionTree, client.v1.exec(&request).await?)?;

    assert_eq!(response.initial_tx_id.hash, hash);

    let lp_out = response.actions.get(1).and_then(|action| match action {
        Action::Amm(AmmAction::ProvideLiquidity(data)) => match data.effects.first() {
            Some(ProvideLiquidityEffect::CbAddLiquidity(effect_data)) => match &effect_data.status {
                ActionStatus::Completed(payload) => match &payload.data {
                    CbAddLiquidityResult::MintLp { lp_out } => Some(lp_out.clone()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    });

    assert_eq!(lp_out, Some("85942".to_string()));

    Ok(())
}
