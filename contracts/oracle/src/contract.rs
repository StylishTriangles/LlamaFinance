use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, PriceResponse};
use crate::query::{query_price, query_prices};
use crate::state::{ITEMS, ADMIN};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    ADMIN.save(deps.storage, &deps.api.addr_validate(&msg.admin)?)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPrice { symbol, precision, price } => {
            if info.sender != ADMIN.load(deps.storage)? {
                return Err(ContractError::Unauthorized {});
            }
            ITEMS.save(deps.storage, &symbol, &PriceResponse {
                symbol: symbol.clone(),
                price,
                precision,
            })?;
            Ok(Response::default())
        }
        ExecuteMsg::AddSymbol { symbol } => {
            if info.sender != ADMIN.load(deps.storage)? {
                return Err(ContractError::Unauthorized {});
            }
            if ITEMS.has(deps.storage, &symbol) {
                return Err(ContractError::SymbolAlreadyExists {symbol});
            }
            ITEMS.save(deps.storage, &symbol, &PriceResponse { symbol: symbol.clone(), price: Uint128::zero(), precision: Uint128::one() })?;
            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Price { symbol } => Ok(to_binary(&query_price(deps, symbol)?)?),
        QueryMsg::Prices {} => Ok(to_binary(&query_prices(deps)?)?),
    }
}
