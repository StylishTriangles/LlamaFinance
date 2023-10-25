use cosmwasm_std::{Deps, Addr, Uint128, Coin};

use crate::{state::{GlobalData, ASSETS, USER_ASSET_INFO, ASSET_INFO, RATE_DENOMINATOR}, error::{ContractResult, ContractError}};

pub fn max_liquidation_value(
    deps: Deps,
    user: &Addr,
    global_data: &GlobalData,
) -> ContractResult<Uint128> {
    let mut debt_coins = vec![];
    let mut collateral_coins = vec![];
    let assets = ASSETS.load(deps.storage)?;
    for asset in assets.iter() {
        let user_key = (user, asset.as_ref());
        if let Ok(user_asset_info) = USER_ASSET_INFO.load(deps.storage, user_key) {
            {
                let amount = user_asset_info.borrow_amount;
                let coin = Coin {
                    denom: asset.clone(),
                    amount,
                };
                debt_coins.push(coin);
            }
            {
                let amount = user_asset_info.collateral;
                let coin = Coin {
                    denom: asset.clone(),
                    amount,
                };
                collateral_coins.push(coin);
            }
        }
    }
    let debt_value = calculate_coins_value(deps, debt_coins)?;
    let collateral_value = calculate_coins_value(deps, collateral_coins)?;
    let no_liquidation = Ok(Uint128::zero());
    if collateral_value.is_zero() {
        return no_liquidation;
    }
    let ltv = debt_value.checked_multiply_ratio(RATE_DENOMINATOR, collateral_value).ok().ok_or(ContractError::LTVTooHigh {  })?;
    if ltv <= Uint128::from(global_data.liquidation_threshold) {
        return no_liquidation;
    }

    let last_safe_debt_value = collateral_value.checked_multiply_ratio(global_data.liquidation_threshold, RATE_DENOMINATOR).ok().ok_or(ContractError::InvalidLiquidationThreshold {  })?;
    let threshold_delta = RATE_DENOMINATOR.checked_sub(global_data.liquidation_threshold).ok_or(ContractError::InvalidLiquidationThreshold {  })?;
    let marginal_unsafe_debt_value = debt_value.checked_sub(last_safe_debt_value).ok().ok_or(ContractError::InvalidDebtValue{})?;
    marginal_unsafe_debt_value.checked_multiply_ratio(RATE_DENOMINATOR, threshold_delta).ok().ok_or(ContractError::InvalidLiquidationThreshold {  })
}

pub fn calculate_coins_value(
    deps: Deps,
    coins: Vec<Coin>,
) -> ContractResult<Uint128> {
    let mut value = Uint128::zero();
    for coin in coins.iter() {
        let asset_info = ASSET_INFO.load(deps.storage, &coin.denom)?;
        let coin_value = coin.amount.checked_multiply_ratio(asset_info.price, asset_info.price_precision).ok().ok_or(ContractError::PriceTooHigh {  })?;
        value = value.checked_add(coin_value).ok().ok_or(ContractError::PriceTooHigh {  })?;
    }
    Ok(value)
}