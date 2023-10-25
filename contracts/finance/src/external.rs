use cosmwasm_std::to_binary;
use cosmwasm_std::{Deps, Uint128, QueryRequest, WasmQuery};

use crate::error::ContractResult;
use oracle::msg::QueryMsg as OracleQueryMsg;

use oracle::msg::PriceResponse;


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
