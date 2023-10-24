use cosmwasm_std::{Deps, Uint128};

use crate::msg::{PriceResponse, PricesResponse};
use crate::error::{ContractError, ContractResult};

use crate::state::PRICES;


pub fn query_price(deps: Deps, symbol: String) -> ContractResult<PriceResponse> {
    match PRICES.may_load(deps.storage, &symbol) {
        Ok(Some(v)) => Ok(PriceResponse { price: v.into() }),
        Ok(None) => Err(ContractError::SymbolNotExists { symbol }),
        Err(_) => Err(ContractError::StorageError {}),
    }
}

pub fn query_prices(deps: Deps) -> ContractResult<PricesResponse> {
    // Iterate over PRICES and collect into a vector
    let prices: Vec<(String, Uint128)> = PRICES
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| {
            let (symbol, price) = item?;
            Ok((symbol, price))
        })
        .collect::<ContractResult<Vec<(String, Uint128)>>>()?;

    Ok(PricesResponse { prices })
}