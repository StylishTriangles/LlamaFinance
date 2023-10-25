use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserAssetInfo {
    pub collateral: Uint128,
    pub borrow_amount: Uint128,
    pub l_asset_amount: Uint128,
    pub cumulative_interest: Uint128,
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
    pub target_utilization_rate_bps: u16,
    pub decimals: u16,
    pub min_rate: u32,
    pub optimal_rate: u32,
    pub max_rate: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    /// For FE
    pub apr: Uint128,
    pub total_deposit: Uint128,
    pub total_borrow: Uint128,
    pub total_l_asset: Uint128,
    pub total_collateral: Uint128,
    pub cumulative_interest: Uint128,
    pub asset_config: AssetConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalData {
    pub last_update: u64,
    pub oracle: Addr,
}

pub const USER_ASSET_INFO: Map<(&Addr, &str), UserAssetInfo> = Map::new("user_asset");
pub const USER_DATA: Map<&Addr, UserData> = Map::new("user_data");
pub const ASSET_INFO: Map<&str, AssetInfo> = Map::new("asset_info");
pub const ASSETS: Item<Vec<String>> = Item::new("assets");
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const GLOBAL_DATA: Item<GlobalData> = Item::new("global_data");
