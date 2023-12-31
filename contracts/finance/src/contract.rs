use std::convert::TryInto;
use crate::external::query_price;
use crate::liquidation::{calculate_coins_value, max_liquidation_value};

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, BankMsg, Coin, Addr,
};
use oracle::msg::PriceResponse;

use crate::error::{ContractError, ContractResult};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::{USER_ASSET_INFO, ASSETS, ASSET_INFO, ADMIN, UserAssetInfo, AssetConfig, AssetInfo, GLOBAL_DATA, GlobalData, RATE_DENOMINATOR, NANOSECONDS_IN_YEAR};
use crate::query::query_handler;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    ADMIN.save(deps.storage, &deps.api.addr_validate(&msg.admin)?)?;
    ASSETS.save(deps.storage, &vec![])?;
    let liquidation_threshold = 70 * RATE_DENOMINATOR / 100; 
    let global_data = GlobalData {
        oracle: msg.oracle,
        liquidation_threshold,
    };
    GLOBAL_DATA.save(deps.storage, &global_data)?;
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
        ExecuteMsg::Liquidate { user_addr, denom } => {
            liquidate(deps, env, info, user_addr, denom)
        },
        ExecuteMsg::UpdateUserAssetInfo { user_addr } => {
            update_user_asset_info(deps, env, user_addr)
        },
        ExecuteMsg::UpdateAsset { denom, decimals, target_utilization_rate_bps, min_rate, optimal_rate, max_rate } => {
            update_asset(deps, env, info, denom, target_utilization_rate_bps, decimals, min_rate, optimal_rate, max_rate)
        }
    }
}

fn deposit_collateral(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    update(&mut deps, &env, &info.sender)?;


    for coin in info.funds.iter() { 
        // Fetch global cumulative interest for this asset
        let mut asset_info = ASSET_INFO.load(deps.storage, &coin.denom)?;

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
    amount: Uint128,
    asset_info: &AssetInfo,
) -> ContractResult<Uint128> {
    if asset_info.total_l_asset.is_zero() { 
        Ok(amount)
    } else {
        amount.checked_multiply_ratio(asset_info.total_l_asset, asset_info.total_deposit).ok().ok_or(ContractError::TooManyLAssets{})
    }
}

fn convert_l_asset_to_asset(
    amount: Uint128,
    asset_info: &AssetInfo,
) -> ContractResult<Uint128> {
    amount.checked_multiply_ratio(asset_info.total_deposit, asset_info.total_l_asset).ok().ok_or(ContractError::TooManyLAssets{})
}

fn calculate_rate(asset_info: &AssetInfo) -> ContractResult<u32> {
    let config = &asset_info.asset_config;
    let util_rate = asset_info.total_borrow.checked_multiply_ratio(RATE_DENOMINATOR, asset_info.total_deposit).ok().ok_or(ContractError::InvalidUtilizationRatio {  })?;

    if let Ok(util_delta) = util_rate.checked_sub(Uint128::from(config.target_utilization_rate_bps)) {
        let max_util_delta = RATE_DENOMINATOR.checked_sub(config.target_utilization_rate_bps).ok_or(ContractError::InvalidTargetUtilization {  })?;
        let rate_delta = config.max_rate.checked_sub(config.optimal_rate).ok_or(ContractError::InvalidMaxRate {  })?;
        let util_delta_rate = util_delta.checked_multiply_ratio(rate_delta, RATE_DENOMINATOR).ok().ok_or(ContractError::InvalidUtilizationRatio {  })?;
        let nonlinear_rate = util_delta_rate.checked_multiply_ratio(RATE_DENOMINATOR, max_util_delta).ok().ok_or(ContractError::InvalidTargetUtilization {  })?;
        nonlinear_rate.u128().try_into().ok().ok_or(ContractError::InvalidTargetUtilization {  })
    } else {
        let rate_delta = config.optimal_rate.checked_sub(config.min_rate).ok_or(ContractError::InvalidOptimalRate {  })?;
        let linear_rate = util_rate.checked_multiply_ratio(rate_delta, config.target_utilization_rate_bps).ok().ok_or(ContractError::InvalidTargetUtilization{})?;
        let rate = linear_rate.checked_add(Uint128::from(config.min_rate)).ok().ok_or(ContractError::InvalidMinRate {  })?;
        rate.u128().try_into().ok().ok_or(ContractError::InvalidMinRate {  })
    }
}

fn update(
    deps: &mut DepsMut,
    env: &Env,
    user: &Addr,
) -> Result<(), ContractError> {
    let now = env.block.time;
    let assets = ASSETS.load(deps.storage)?;
    for denom in assets.iter() {
        let mut asset_info = ASSET_INFO.load(deps.storage, denom)?;
        let rate = if asset_info.total_deposit.is_zero() {
            asset_info.asset_config.min_rate
        } else {
            calculate_rate(&asset_info)?
        };
        let time_elapsed = now.nanos().checked_sub(asset_info.last_update.nanos()).ok_or(ContractError::ClockSkew {  })?;
        let new_interest_after_year = asset_info.cumulative_interest.checked_multiply_ratio(rate, RATE_DENOMINATOR).ok().ok_or(ContractError::InvalidRate {  })?;
        let new_interest = new_interest_after_year.checked_multiply_ratio(time_elapsed, NANOSECONDS_IN_YEAR).ok().ok_or(ContractError::InvalidTimeElapsed{})?;
        let final_cumulative_rate = asset_info.cumulative_interest.saturating_add(new_interest);

        let user_key = (user, denom.as_ref());
        if let Ok(mut user_asset_info) = USER_ASSET_INFO.load(deps.storage, user_key) {
            let final_borrow_amount = user_asset_info.borrow_amount.checked_multiply_ratio(final_cumulative_rate, asset_info.cumulative_interest).ok().ok_or(ContractError::InvalidCumulativeInterest{})?;
            user_asset_info.borrow_amount = final_borrow_amount;
            USER_ASSET_INFO.save(deps.storage, user_key, &user_asset_info)?;
        }
        let final_total_borrow = asset_info.total_borrow.checked_multiply_ratio(final_cumulative_rate, asset_info.cumulative_interest).ok().ok_or(ContractError::InvalidCumulativeInterest{})?;
        let new_deposit = final_total_borrow.saturating_sub(asset_info.total_borrow);
        let final_total_deposit = asset_info.total_deposit.saturating_add(new_deposit);

        asset_info.cumulative_interest = final_cumulative_rate;
        asset_info.total_borrow = final_total_borrow;
        asset_info.total_deposit = final_total_deposit;
        asset_info.last_update = now;
        asset_info.apr = Uint128::from(rate);
        ASSET_INFO.save(deps.storage, denom, &asset_info)?;
    }
    Ok(())
}

fn update_user_asset_info(
    mut deps: DepsMut,
    env: Env,
    user_addr: String,
) -> ContractResult<Response> {
    let user = deps.api.addr_validate(&user_addr)?;
    update(&mut deps, &env, &user)?;
    Ok(Response::default())
}

fn deposit(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> ContractResult<Response> {
    update(&mut deps, &env, &info.sender)?;


    for coin in info.funds.iter() {
        // Fetch global cumulative interest for this asset
        let mut asset_info = ASSET_INFO.load(deps.storage, &coin.denom)?;
        let l_asset_amt = convert_asset_to_l_asset(coin.amount, &asset_info)?;


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


fn withdraw(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    update(&mut deps, &env, &info.sender)?;


    let mut asset_info = ASSET_INFO.load(deps.storage, &denom)?;
    let user_addr = info.sender.clone();
    let asset_amount = convert_l_asset_to_asset(amount, &asset_info)?;

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

    let response = Response::new().add_message(BankMsg::Send {
        to_address: info.sender.clone().into(),
        amount: vec![
            Coin {
                amount: asset_amount,
                denom,
            }
        ]
    });
    Ok(response)
}

fn withdraw_collateral(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> ContractResult<Response> {
    update(&mut deps, &env, &info.sender)?;


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
        to_address: info.sender.clone().into(),
        amount: vec![
            Coin {
                amount,
                denom,
            }
        ]
    });
    validate_ltv(deps, &info)?;
    Ok(response)
}

fn validate_ltv(
    mut deps: DepsMut,
    info: &MessageInfo,
) -> ContractResult<()> {
    let global_data = GLOBAL_DATA.load(deps.storage)?;
    update_prices(&mut deps, &global_data)?;
    let liquidaton_value = max_liquidation_value(deps.as_ref(), &info.sender, &global_data)?;
    if liquidaton_value.is_zero() {
        Ok(())
    } else {
        Err(ContractError::UnsafeLTV {  })
    }
}

fn borrow(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> ContractResult<Response> {
    update(&mut deps, &env, &info.sender)?;

    let mut asset_info = ASSET_INFO.load(deps.storage, &denom)?;

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
        to_address: info.sender.clone().into(),
        amount: vec![
            Coin {
                amount,
                denom,
            }
        ]
    });

    validate_ltv(deps, &info)?;
    Ok(response)
}

fn repay(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> ContractResult<Response> {
    update(&mut deps, &env, &info.sender)?;

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
    if !coins_to_return.is_empty() {
        response = response.add_message(BankMsg::Send {
            to_address: info.sender.into(),
            amount: coins_to_return,
        });
    } 

    Ok(response)
}


fn update_prices(
    deps: &mut DepsMut,
    global_data: &GlobalData,
) -> ContractResult<()> {
    let assets = ASSETS.load(deps.storage)?;
    for denom in assets.iter() {
        let PriceResponse {
            price,
            symbol: _,
            precision,
        } = query_price(deps.as_ref(), global_data.oracle.clone(), denom.clone())?;
        let mut asset_info = ASSET_INFO.load(deps.storage, denom)?;
        asset_info.price = price;
        asset_info.price_precision = precision;
        ASSET_INFO.save(deps.storage, denom, &asset_info)?;
    }
    Ok(())
}

fn liquidate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user_addr: String,
    denom: String,
) -> ContractResult<Response> {
    if info.funds.len() != 1 {
        return Err(ContractError::MultiLiquidateNotSupprted{});
    }
    let debt_coin = info.funds[0].clone();
    update(&mut deps, &env, &info.sender)?;
    let global_data = GLOBAL_DATA.load(deps.storage)?;
    update_prices(&mut deps, &global_data)?;

    let user = deps.api.addr_validate(&user_addr)?;

    let repay_value = calculate_coins_value(deps.as_ref(), info.funds)?;
    let max_payment = max_liquidation_value(deps.as_ref(), &user, &global_data)?;
    
    let extra_repay_value = repay_value.saturating_sub(max_payment);
    let final_repay_value = repay_value.checked_sub(extra_repay_value).ok().ok_or(ContractError::InvalidExtraRepayValue {  })?;

    let mut withdraw_asset_info = ASSET_INFO.load(deps.storage, &denom)?;
    let mut debt_asset_info = ASSET_INFO.load(deps.storage, &debt_coin.denom)?;
    let final_withdraw_amount = final_repay_value.checked_multiply_ratio(withdraw_asset_info.price_precision, withdraw_asset_info.price).ok().ok_or(ContractError::InvalidPrice{})?;
    

    let response = Response::new().add_message(BankMsg::Send {
        to_address: info.sender.into(),
        amount: vec![
            Coin {
                amount: final_withdraw_amount,
                denom: denom.clone(),
            }
        ]
    });
    let user_key = (&user, denom.as_str());
    let mut user_asset_info = USER_ASSET_INFO.load(deps.storage, user_key)?;
    let user_debt_key = (&user, debt_coin.denom.as_str());
    let mut user_debt_asset_info = USER_ASSET_INFO.load(deps.storage, user_debt_key)?;

    let final_user_collateral = user_asset_info.collateral.checked_sub(final_withdraw_amount).ok().ok_or(ContractError::InvalidFinalWithdrawAmount {  })?;
    let final_user_debt_borrow = user_debt_asset_info.borrow_amount.checked_sub(debt_coin.amount).ok().ok_or(ContractError::InvalidTotalBorrow {  })?;
    let final_total_collateral = withdraw_asset_info.total_collateral.checked_sub(final_withdraw_amount).ok().ok_or(ContractError::InvalidFinalWithdrawAmount {  })?;
    let final_debt_total_borrowed = debt_asset_info.total_borrow.checked_sub(debt_coin.amount).ok().ok_or(ContractError::InvalidTotalBorrow {  })?;
    
    user_debt_asset_info.borrow_amount = final_user_debt_borrow;
    USER_ASSET_INFO.save(deps.storage, user_key, &user_debt_asset_info)?;
    
    user_asset_info.collateral = final_user_collateral;
    USER_ASSET_INFO.save(deps.storage, user_key, &user_asset_info)?;
    
    withdraw_asset_info.total_collateral = final_total_collateral;
    ASSET_INFO.save(deps.storage, &denom, &withdraw_asset_info)?;
    
    debt_asset_info.total_borrow = final_debt_total_borrowed;
    ASSET_INFO.save(deps.storage, &debt_coin.denom, &debt_asset_info)?;

    Ok(response)
}

#[allow(clippy::too_many_arguments)]
fn update_asset(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    target_utilization_rate_bps: u32,
    decimals: u16,
    min_rate: u32,
    optimal_rate: u32,
    max_rate: u32,
) -> ContractResult<Response> {
    if ADMIN.load(deps.storage)? != info.sender {
        return Err(ContractError::Unauthorized {  });
    }

    let mut new_asset = false;
    ASSET_INFO.update(deps.storage, &denom, 
        |asset_info| -> ContractResult<AssetInfo> {
            match asset_info {
                Some(mut asset_info) => {
                    asset_info.asset_config = AssetConfig {
                        target_utilization_rate_bps,
                        decimals,
                        min_rate,
                        optimal_rate,
                        max_rate,
                    };
                    Ok(asset_info)
                },
                None => {
                    new_asset = true;
                    Ok(
                        AssetInfo {
                            denom: denom.clone(),
                            apr: Uint128::zero(),
                            total_deposit: Uint128::zero(),
                            total_borrow: Uint128::zero(),
                            total_l_asset: Uint128::zero(),
                            total_collateral: Uint128::zero(),
                            cumulative_interest: Uint128::from(1u128<<64),
                            last_update: env.block.time,
                            asset_config: AssetConfig {
                                target_utilization_rate_bps,
                                decimals,
                                min_rate,
                                optimal_rate,
                                max_rate,
                            },
                            price: Uint128::zero(),
                            price_precision: Uint128::zero(),
                        }
                    )
                }
            }
        }
    )?;

    if new_asset {
        let mut assets = ASSETS.load(deps.storage)?;
        assets.push(denom);
        ASSETS.save(deps.storage, &assets)?;
    }
    update(&mut deps, &env, &env.contract.address)?;

    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    query_handler(deps, env, msg)
}

