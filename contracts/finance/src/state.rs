use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Vec<Coin>,
    pub counter_offer: Vec<Coin>,
    pub expires: u64,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<State> = Item::new(CONFIG_KEY);


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserAssetInfo {
    pub collateral: Uint128,
    pub loan_amount: Uint128,
    pub cumulative_interest: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserData {
    pub last_update_time: u64,
    pub ltv: Uint128,
    pub assets_value: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssetInfo {
    /// For FE
    pub apr: Uint128,
    pub total_dep: Uint128,
    pub total_fasset: Uint128,
    pub cumulative_interest: Uint128,
}


pub const USER_ASSET: Map<(&Addr, &str), UserAssetInfo> = Map::new("user_asset");
pub const USER_DATA: Map<&Addr, UserData> = Map::new("user_data");
pub const ASSET_INFO: Map<&str, AssetInfo> = Map::new("asset_info");
pub const ASSETS: Item<Vec<String>> = Item::new("assets");
