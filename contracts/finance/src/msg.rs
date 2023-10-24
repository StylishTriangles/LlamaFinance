use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

