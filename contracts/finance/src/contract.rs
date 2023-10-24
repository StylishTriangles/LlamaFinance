use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, BankMsg, Coin,
};

use crate::error::{ContractError, ContractResult};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::{USER_ASSET_INFO, ASSETS, ASSET_INFO, ADMIN, UserAssetInfo, AssetConfig, AssetInfo};
use crate::query::query_handler;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> ContractResult<Response> {
    ADMIN.save(deps.storage, &info.sender)?;
    ASSETS.save(deps.storage, &vec![])?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::Deposit {} => {
            deposit(deps, env, info)
        },
        ExecuteMsg::Withdraw { denom, amount } => {
            withdraw(deps, env, info, denom, amount)
        },
        ExecuteMsg::DepositCollateral {} => {
            deposit_collateral(deps, env, info)
        },
        ExecuteMsg::WithdrawCollateral { denom, amount } => {
            withdraw_collateral(deps, env, info, denom, amount)
        },
        ExecuteMsg::Borrow { denom, amount } => {
            borrow(deps, env, info, denom, amount)
        },
        ExecuteMsg::Repay {} => {
            repay(deps, env, info)
        },
        ExecuteMsg::UpdateAsset { denom, decimals, target_utilization_rate_bps, k0, k1, k2 } => {
            update_asset(deps, env, info, denom, decimals, target_utilization_rate_bps, k0, k1, k2)
        }
    }
}

fn deposit_collateral(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // TODO: update global and user state


    for coin in info.funds.iter() {
        if !check_asset(deps.as_ref(), coin.denom.clone()) {
            return Err(ContractError::InvalidAssetDeposit {  });
        }
        // Fetch global cumulative interest for this asset
        let mut asset_info = ASSET_INFO.load(deps.storage, &coin.denom).unwrap();

        USER_ASSET_INFO.update(
            deps.storage, 
            (&info.sender, &coin.denom),
            |user_asset| -> StdResult<UserAssetInfo> {
                match user_asset {
                    Some(mut user_asset) => {
                        user_asset.collateral += coin.amount;
                        Ok(user_asset)
                    },
                    None => {
                        Ok(UserAssetInfo {
                            collateral: coin.amount,
                            borrow_amount: Uint128::zero(),
                            l_asset_amount: Uint128::zero(),
                            cumulative_interest: asset_info.cumulative_interest,
                        })
                    }
                }
            }
        )?;

        asset_info.total_collateral += coin.amount;

        ASSET_INFO.save(deps.storage, &coin.denom, &asset_info)?;
    }

    Ok(Response::default())
}

fn convert_asset_to_l_asset(
    amount_asset: Uint128,
    total_deposit: Uint128,
    total_l_asset: Uint128,
) -> Uint128 {
    amount_asset.checked_multiply_ratio(total_l_asset, total_deposit).unwrap()
}

fn convert_l_asset_to_asset(
    amount_l_asset: Uint128,
    total_deposit: Uint128,
    total_l_asset: Uint128,
) -> Uint128 {
    amount_l_asset.checked_multiply_ratio(total_deposit, total_l_asset).unwrap()
}


fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> ContractResult<Response> {
    // TODO: update global and user state


    for coin in info.funds.iter() {
        if !check_asset(deps.as_ref(), coin.denom.clone()) {
            return Err(ContractError::InvalidAssetDeposit {  });
        }
        // Fetch global cumulative interest for this asset
        let mut asset_info = ASSET_INFO.load(deps.storage, &coin.denom).unwrap();
        let l_asset_amt = convert_asset_to_l_asset(coin.amount, asset_info.total_deposit, asset_info.total_l_asset);


        USER_ASSET_INFO.update(
            deps.storage, 
            (&info.sender, &coin.denom),
            |user_asset| -> ContractResult<UserAssetInfo> {
                match user_asset {
                    Some(mut user_asset) => {
                        user_asset.l_asset_amount += l_asset_amt;
                        Ok(user_asset)
                    },
                    None => {
                        Ok(UserAssetInfo {
                            collateral: Uint128::zero(),
                            borrow_amount: Uint128::zero(),
                            l_asset_amount: coin.amount,
                            cumulative_interest: asset_info.cumulative_interest,
                        })
                    }
                }
            }
        )?;

        asset_info.total_deposit += coin.amount;
        asset_info.total_l_asset += l_asset_amt;

        ASSET_INFO.save(deps.storage, &coin.denom, &asset_info)?;
    }

    Ok(Response::default())
}

fn check_asset(deps: Deps, asset: String) -> bool {
    let assets = ASSETS.load(deps.storage).unwrap();
    assets.contains(&asset)
}

fn withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // TODO: update global and user state


    let mut asset_info = ASSET_INFO.load(deps.storage, &denom)?;
    let user_addr = info.sender;
    let asset_amount = convert_l_asset_to_asset(amount, asset_info.total_deposit, asset_info.total_l_asset);

    USER_ASSET_INFO.update(
        deps.storage, 
        (&user_addr, &denom),
        |user_asset| -> ContractResult<UserAssetInfo> {
            match user_asset {
                Some(mut user_asset) => {
                    user_asset.l_asset_amount = user_asset.l_asset_amount.checked_sub(amount).ok().ok_or(ContractError::InsufficientFunds {})?;
                    Ok(user_asset)
                },
                None => {
                    Err(ContractError::UserDoesntHaveAsset {})
                }
            }
        }
    )?;

    asset_info.total_l_asset = asset_info.total_l_asset.checked_sub(amount).ok().ok_or(ContractError::InsufficientFunds {})?;
    asset_info.total_deposit = asset_info.total_deposit.checked_sub(asset_amount).ok().ok_or(ContractError::InsufficientFunds {})?;

    // Don't allow to withdraw more than is available
    if asset_info.total_deposit < asset_info.total_borrow {
        return Err(ContractError::InsufficientFunds {});
    }

    ASSET_INFO.save(deps.storage, &denom, &asset_info)?;

    // TODO: update asset APR

    Ok(Response::default())
}

fn withdraw_collateral(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> ContractResult<Response> {
    // TODO: update global and user state


    let mut asset_info = ASSET_INFO.load(deps.storage, &denom)?;
    let user_addr = info.sender.clone();

    // TODO: check collateralization ratio

    USER_ASSET_INFO.update(
        deps.storage, 
        (&user_addr, &denom),
        |user_asset| -> ContractResult<UserAssetInfo> {
            match user_asset {
                Some(mut user_asset) => {
                    user_asset.collateral = user_asset.collateral.checked_sub(amount).ok().ok_or(ContractError::InsufficientFunds {})?;
                    Ok(user_asset)
                },
                None => {
                    Err(ContractError::UserDoesntHaveAsset {})
                }
            }
        }
    )?;

    asset_info.total_collateral = asset_info.total_collateral.checked_sub(amount).ok().ok_or(ContractError::InsufficientFunds {})?;

    ASSET_INFO.save(deps.storage, &denom, &asset_info)?;

    let response = Response::new().add_message(BankMsg::Send {
        to_address: info.sender.into(),
        amount: vec![
            Coin {
                amount,
                denom,
            }
        ]
    });
    Ok(response)
}

fn borrow(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> ContractResult<Response> {
    // TODO: update global and user state

    let mut asset_info = ASSET_INFO.load(deps.storage, &denom)?;

    // TODO: check collateralization ratio

    USER_ASSET_INFO.update(deps.storage, (&info.sender, &denom), |user_asset| -> ContractResult<UserAssetInfo> {
        match user_asset {
            Some(mut user_asset) => {
                user_asset.borrow_amount += amount;
                Ok(user_asset)
            },
            None => {
                Ok(UserAssetInfo {
                    collateral: Uint128::zero(),
                    borrow_amount: amount,
                    l_asset_amount: Uint128::zero(),
                    cumulative_interest: asset_info.cumulative_interest,
                })
            }
        }
    })?;

    asset_info.total_borrow += amount;

    if asset_info.total_deposit < asset_info.total_borrow {
        return Err(ContractError::InsufficientFunds {});
    }

    ASSET_INFO.save(deps.storage, &denom, &asset_info)?;

    let response = Response::new().add_message(BankMsg::Send {
        to_address: info.sender.into(),
        amount: vec![
            Coin {
                amount,
                denom,
            }
        ]
    });

    Ok(response)
}

fn repay(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> ContractResult<Response> {
    // TODO: update global and user state

    let mut coins_to_return = vec![];

    for coin in info.funds.iter() {
        let mut asset_info = ASSET_INFO.load(deps.storage, &coin.denom)?;
        let mut dust: Uint128 = Uint128::zero();

        USER_ASSET_INFO.update(deps.storage, (&info.sender, &coin.denom), |user_asset| -> ContractResult<UserAssetInfo> {
            match user_asset {
                Some(mut user_asset) => {
                    if user_asset.borrow_amount < coin.amount {
                        dust = coin.amount - user_asset.borrow_amount;
                        user_asset.borrow_amount = Uint128::zero();
                    } else {
                        user_asset.borrow_amount -= coin.amount;
                    }
                    Ok(user_asset)
                },
                None => {
                    Err(ContractError::UserDoesntHaveAsset {})
                }
            }
        })?;

        asset_info.total_borrow = asset_info.total_borrow.saturating_sub(coin.amount);

        ASSET_INFO.save(deps.storage, &coin.denom, &asset_info)?;

        if dust > Uint128::zero() {
            coins_to_return.push(Coin {
                amount: dust,
                denom: coin.denom.clone(),
            });
        }
    }

    let mut response = Response::new();
    if coins_to_return.len() > 0 {
        response = response.add_message(BankMsg::Send {
            to_address: info.sender.into(),
            amount: coins_to_return,
        });
    } 

    Ok(response)
}

fn update_asset(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    denom: String,
    target_utilization_rate_bps: u16,
    decimals: u16,
    k0: Uint128,
    k1: Uint128,
    k2: Uint128,
) -> ContractResult<Response> {
    let mut new_asset = false;
    ASSET_INFO.update(deps.storage, &denom, 
        |asset_info| -> ContractResult<AssetInfo> {
            match asset_info {
                Some(mut asset_info) => {
                    asset_info.asset_config = AssetConfig {
                        target_utilization_rate_bps,
                        decimals,
                        k0,
                        k1,
                        k2,
                    };
                    Ok(asset_info)
                },
                None => {
                    new_asset = true;
                    Ok(
                        AssetInfo {
                            apr: Uint128::zero(),
                            total_deposit: Uint128::zero(),
                            total_borrow: Uint128::zero(),
                            total_l_asset: Uint128::zero(),
                            total_collateral: Uint128::zero(),
                            cumulative_interest: Uint128::from(1u128<<64),
                            asset_config: AssetConfig {
                                target_utilization_rate_bps,
                                decimals,
                                k0,
                                k1,
                                k2,
                            },
                        }
                    )
                }
            }
        }
    )?;

    if new_asset {
        let mut assets = ASSETS.load(deps.storage)?;
        assets.push(denom.clone());
        ASSETS.save(deps.storage, &assets)?;
    }

    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    query_handler(deps, env, msg)
}

