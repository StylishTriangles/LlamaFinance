use cosmwasm_std::{
    Binary, Deps, Env, to_binary
};

use crate::error::ContractResult;
use crate::msg::QueryMsg;

use crate::state::{USER_ASSET_INFO, ASSETS, ASSET_INFO, UserAssetInfo, USER_DATA, UserData, AssetInfo};


pub fn query_handler(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    let raw = match msg {
        QueryMsg::Assets {} => to_binary(&query_assets(deps)?),
        QueryMsg::UserAssetsInfo { user } => to_binary(&query_user_assets_info(deps, user)?),
        QueryMsg::UserAssetInfo { user, denom } => to_binary(&query_user_asset_info(deps, user, denom)?),
        QueryMsg::UserData { user } => to_binary(&query_user_data(deps, user)?),
        QueryMsg::AssetInfo { denom } => to_binary(&query_asset_info(deps, denom)?),
        QueryMsg::AssetsInfo {} => to_binary(&query_assets_info(deps)?),
    }?;
    Ok(raw)
}

pub fn query_assets(deps: Deps) -> ContractResult<Vec<String>> {
    Ok(ASSETS.load(deps.storage)?)
}

pub fn query_user_asset_info(deps: Deps, user: String, denom: String) -> ContractResult<UserAssetInfo> {
    let sender = deps.api.addr_validate(&user)?;
    let user_asset_info = USER_ASSET_INFO.load(deps.storage, (&sender, &denom))?;
    Ok(user_asset_info)
}

pub fn query_user_assets_info(deps: Deps, user: String) -> ContractResult<Vec<UserAssetInfo>> {
    let addr = deps.api.addr_validate(&user)?;
    let assets = ASSETS.load(deps.storage)?;
    let mut user_assets_info: Vec<UserAssetInfo> = Vec::new();
    for asset in assets {
        let user_asset_info = USER_ASSET_INFO.may_load(deps.storage, (&addr, &asset))?.unwrap_or_default();
        user_assets_info.push(user_asset_info);
    }
    Ok(user_assets_info)
}

pub fn query_user_data(deps: Deps, user: String) -> ContractResult<UserData> {
    let sender = deps.api.addr_validate(&user)?;
    let user_data = USER_DATA.load(deps.storage, &sender)?;
    Ok(user_data)
}

pub fn query_asset_info(deps: Deps, denom: String) -> ContractResult<AssetInfo> {
    let asset_info = ASSET_INFO.load(deps.storage, &denom)?;
    Ok(asset_info)
}

pub fn query_assets_info(deps: Deps) -> ContractResult<Vec<AssetInfo>> {
    let assets = ASSETS.load(deps.storage)?;
    let mut assets_info: Vec<AssetInfo> = Vec::new();
    for asset in assets {
        let asset_info = ASSET_INFO.load(deps.storage, &asset)?;
        assets_info.push(asset_info);
    }
    Ok(assets_info)
}