use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Timestamp};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserAssetInfo {
    pub collateral: Uint128,
    pub borrow_amount: Uint128,
    pub l_asset_amount: Uint128,
    pub cumulative_interest: Uint128,
}

impl Default for UserAssetInfo {
    fn default() -> Self {
        UserAssetInfo {
            collateral: Uint128::zero(),
            borrow_amount: Uint128::zero(),
            l_asset_amount: Uint128::zero(),
            cumulative_interest: Uint128::from(1u128<<64),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub ltv: Uint128,
    pub assets_value: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssetConfig {
    pub target_utilization_rate_bps: u32,
    pub decimals: u16,
    pub min_rate: u32,
    pub optimal_rate: u32,
    pub max_rate: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    /// For FE
    pub denom: String,
    pub apr: Uint128,
    pub total_deposit: Uint128,
    pub total_borrow: Uint128,
    pub total_l_asset: Uint128,
    pub total_collateral: Uint128,
    pub cumulative_interest: Uint128,
    pub asset_config: AssetConfig,
    pub last_update: Timestamp,
    pub price: Uint128,
    pub price_precision: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalData {
    pub oracle: String,
    pub liquidation_threshold: u32,
}

pub const USER_ASSET_INFO: Map<(&Addr, &str), UserAssetInfo> = Map::new("user_asset");
pub const USER_DATA: Map<&Addr, UserData> = Map::new("user_data");
pub const ASSET_INFO: Map<&str, AssetInfo> = Map::new("asset_info");
pub const ASSETS: Item<Vec<String>> = Item::new("assets");
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const GLOBAL_DATA: Item<GlobalData> = Item::new("global_data");
pub const RATE_DENOMINATOR: u32 = 10_000;
pub const NANOSECONDS_IN_YEAR: u64 = 365 * 24 * 60 * 60 * 1_000_000_000;
