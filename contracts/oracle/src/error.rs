use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Symbol does not exist (symbol {symbol})")]
    SymbolNotExists { symbol: String },

    #[error("Error (de)serializing storage")]
    StorageError,
}

/// ContractResult is Result<T, ContractError>
pub type ContractResult<T> = Result<T, ContractError>;
