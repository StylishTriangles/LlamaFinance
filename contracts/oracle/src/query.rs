use cosmwasm_std::Deps;

use crate::msg::PriceResponse;
use crate::error::{ContractError, ContractResult};

use crate::state::ITEMS;


pub fn query_price(deps: Deps, symbol: String) -> ContractResult<PriceResponse> {
    match ITEMS.may_load(deps.storage, &symbol) {
        Ok(Some(v)) => Ok(v),
        Ok(None) => Err(ContractError::SymbolNotExists { symbol }),
        Err(_) => Err(ContractError::StorageError {}),
    }
}

pub fn query_prices(deps: Deps) -> ContractResult<Vec<PriceResponse>> {
    // Iterate over PRICES and collect into a vector
    let prices: Vec<PriceResponse> = ITEMS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| {
            let item = item?;
            Ok(item.1)
        })
        .collect::<ContractResult<Vec<PriceResponse>>>()?;

    Ok(prices)
}