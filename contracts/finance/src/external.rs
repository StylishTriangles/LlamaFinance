use cosmwasm_std::to_binary;
use cosmwasm_std::{Deps, QueryRequest, WasmQuery};

use crate::error::ContractResult;
use oracle::msg::QueryMsg as OracleQueryMsg;

use oracle::msg::PriceResponse;

#[cfg(not(test))]
pub fn query_price(deps: Deps, oracle_addr: String, denom: String) -> ContractResult<PriceResponse> {
    let msg = QueryRequest::Wasm(
        WasmQuery::Smart { 
            contract_addr: 
            oracle_addr, 
            msg: to_binary(&OracleQueryMsg::Price { symbol: denom })?
        }
    );
    let response: PriceResponse = deps.querier.query(&msg)?;
    Ok(response)
}
#[cfg(test)]
pub fn query_price(_deps: Deps, _oracle_addr: String, denom: String) -> ContractResult<PriceResponse> {
    use cosmwasm_std::Uint128;

    if denom.as_str().starts_with("usdc") {
        Ok(PriceResponse { symbol: denom, price: Uint128::new(1_000_000), precision: Uint128::new(1_000_000) })
    } else {
        Ok(PriceResponse { symbol: denom, price: Uint128::new(35_000_000_000), precision: Uint128::new(1_000_000) })
    }
}

pub fn query_prices(deps: Deps, oracle_addr: String) -> ContractResult<Vec<PriceResponse>> {
    let msg = QueryRequest::Wasm(
        WasmQuery::Smart { 
            contract_addr: 
            oracle_addr, 
            msg: to_binary(&OracleQueryMsg::Prices {})? 
        }
    );
    let response: Vec<PriceResponse> = deps.querier.query(&msg)?;
    Ok(response)
}
