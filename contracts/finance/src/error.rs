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

}

pub type ContractResult<T> = Result<T, ContractError>;
