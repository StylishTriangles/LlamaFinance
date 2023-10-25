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
    UpdateAsset {
        denom: String,
        decimals: u16,
        target_utilization_rate_bps: u16,
        k0: Uint128,
        k1: Uint128,
        k2: Uint128,
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
