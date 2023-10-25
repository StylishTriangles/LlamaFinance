use cosmwasm_std::Deps;

use crate::msg::PriceResponse;
use crate::error::{ContractError, ContractResult};

use crate::state::PRICES;


pub fn query_price(deps: Deps, symbol: String) -> ContractResult<PriceResponse> {
    match PRICES.may_load(deps.storage, &symbol) {
        Ok(Some(v)) => Ok(PriceResponse { symbol, price: v.into() }),
        Ok(None) => Err(ContractError::SymbolNotExists { symbol }),
        Err(_) => Err(ContractError::StorageError {}),
    }
}

pub fn query_prices(deps: Deps) -> ContractResult<Vec<PriceResponse>> {
    // Iterate over PRICES and collect into a vector
    let prices: Vec<PriceResponse> = PRICES
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| {
            let (symbol, price) = item?;
            Ok(PriceResponse {
                symbol,
                price,
            })
        })
        .collect::<ContractResult<Vec<PriceResponse>>>()?;

    Ok(prices)
}