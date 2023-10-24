use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Uint128,
};
use cw_storage_plus::Map;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{UserState, USER};



#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => {
            deposit(deps, env, info)
        },
        ExecuteMsg::Withdraw { amount } => {
            withdraw(deps, env, info, amount)
        }
    }
}

fn deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {   
    if info.funds.len() != 1 {
        return Err(ContractError::Unauthorized {  });
    }
    let coin = info.funds.first().unwrap();
    let r = USER.load(&deps.storage, k);
    Ok(Response::default())
}

fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: u128,
) -> Result<Response, ContractError> {   
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(Binary::default())
}

