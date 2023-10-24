use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_price, query_prices};
use crate::state::{PRICES, ADMIN};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    for key in msg.keys {
        PRICES.save(deps.storage, &key, &Uint128::zero())?;
    }
    ADMIN.save(deps.storage, &_info.sender)?;
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
        ExecuteMsg::SetPrice { symbol, price } => {
            if info.sender != ADMIN.load(deps.storage)? {
                return Err(ContractError::Unauthorized {});
            }
            PRICES.save(deps.storage, &symbol, &price)?;
            Ok(Response::default())
        }
        ExecuteMsg::AddSymbol { symbol } => {
            if info.sender != ADMIN.load(deps.storage)? {
                return Err(ContractError::Unauthorized {});
            }
            PRICES.save(deps.storage, &symbol, &Uint128::zero())?;
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
