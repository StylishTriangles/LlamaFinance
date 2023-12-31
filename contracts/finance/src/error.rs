use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("expired option (expired {expired:?})")]
    OptionExpired { expired: u64 },

    #[error("not expired option (expires {expires:?})")]
    OptionNotExpired { expires: u64 },

    #[error("unauthorized")]
    Unauthorized {},

    #[error("must send exact counter offer (offer {offer:?}, counter_offer: {counter_offer:?})")]
    CounterOfferMismatch {
        offer: Vec<Coin>,
        counter_offer: Vec<Coin>,
    },

    #[error("do not send funds with burn")]
    FundsSentWithBurn {},

    #[error("tried to deposit invalid asset")]
    InvalidAssetDeposit {},

    #[error("insufficient funds")]
    InsufficientFunds {},

    #[error("user doesn't have this asset")]
    UserDoesntHaveAsset {},

    #[error("clock skew")]
    ClockSkew {},

    #[error("invalid rate")]
    InvalidRate {},

    #[error("invalid cumulative interest")]
    InvalidCumulativeInterest {},

    #[error("invalid time elapsed")]
    InvalidTimeElapsed {},

    #[error("invalid total borrow")]
    InvalidTotalBorrow {},

    #[error("invalid optimal rate")]
    InvalidOptimalRate {},

    #[error("invalid max rate")]
    InvalidMaxRate {},

    #[error("invalid utilization ratio")]
    InvalidUtilizationRatio {},

    #[error("invalid target utilization")]
    InvalidTargetUtilization {},

    #[error("invalid min rate")]
    InvalidMinRate {},

    #[error("invalid liquidation threshold")]
    InvalidLiquidationThreshold {},

    #[error("invalid debt value")]
    InvalidDebtValue {},

    #[error("too many l assets")]
    TooManyLAssets {},

    #[error("price too high")]
    PriceTooHigh {},

    #[error("invalid price")]
    InvalidPrice {},

    #[error("ltv too high")]
    LTVTooHigh {},

    #[error("multi liquidate not supported")]
    MultiLiquidateNotSupprted {},

    #[error("invalid extra repay value")]
    InvalidExtraRepayValue {},


    #[error("invalid final withdraw amount")]
    InvalidFinalWithdrawAmount {},

    #[error("unsafe ltv")]
    UnsafeLTV {},
}

pub type ContractResult<T> = Result<T, ContractError>;
