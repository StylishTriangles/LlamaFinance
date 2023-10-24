use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, BankMsg, Coin,
};

use crate::error::{ContractError, ContractResult};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::{USER_ASSET_INFO, ASSETS, ASSET_INFO, UserAssetInfo};


pub fn query_handler(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    Ok(Binary::default())
}