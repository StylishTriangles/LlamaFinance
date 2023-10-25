use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub oracle: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ExecuteMsg {
    Deposit {},
    Withdraw {
        denom: String,
        amount: Uint128,
    },
    DepositCollateral {},
    WithdrawCollateral {
        denom: String,
        amount: Uint128,
    },
    Borrow {
        denom: String,
        amount: Uint128,
    },
    Repay {},
    Liquidate {
        user_addr: String,
        denom: String,
    },
    UpdateUserAssetInfo {
        user_addr: String, 
    },
    UpdateAsset {
        denom: String,
        decimals: u16,
        target_utilization_rate_bps: u32,
        min_rate: u32,
        optimal_rate: u32,
        max_rate: u32,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum QueryMsg {
    Assets {},
    UserAssetsInfo {
        user: String
    },
    UserAssetInfo {
        user: String,
        denom: String,
    },
    UserData {
        user: String,
    },
    AssetInfo {
        denom: String,
    },
    AssetsInfo {},
}
