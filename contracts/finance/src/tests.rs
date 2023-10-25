use std::str::FromStr;

use cosmwasm_std::{testing::{mock_dependencies_with_balances, mock_env, mock_info}, Addr, Coin, Uint128};

use crate::{msg::{InstantiateMsg, ExecuteMsg}, contract::{instantiate, execute}, state::RATE_DENOMINATOR};

fn rate(x: u32) -> u32 {
    x * RATE_DENOMINATOR / 100
}

#[test]
fn first() -> Result<(), String> {
    let oracle: String = Addr::unchecked("oracle").into();
    let creator: String = Addr::unchecked("creator").into();
    let alice: String = Addr::unchecked("alice").into();
    let btc: String = Addr::unchecked("btc").into();
    let usdc: String = Addr::unchecked("btc").into();
    let alice_balances = &[
        Coin { denom: btc.clone(), amount: Uint128::new(10_000), },
    ];
    let balances = [
        (alice.as_str(), alice_balances),
    ];
    let mut deps = mock_dependencies_with_balances(&balances);
    {
        let msg = InstantiateMsg {
            oracle: oracle.clone(),
            admin: creator.clone(),
        };
        let env = mock_env();
        let info = mock_info(&creator, &[]);
        instantiate(deps.as_mut(), env, info, msg).map_err(|e|format!("instantiate: {}", e))?;
    }
    let btc_rate = rate(80);
    let btc_min_rate = rate(10);
    let btc_optimal_rate = rate(50);
    let btc_max_rate = rate(100);
    {
        let msg = ExecuteMsg::UpdateAsset { 
            denom: btc, 
            decimals: 8, 
            target_utilization_rate_bps: btc_rate, 
            min_rate: btc_min_rate, 
            optimal_rate: btc_optimal_rate, 
            max_rate: btc_max_rate, 
        }; 
        let env = mock_env();
        let info = mock_info(&creator, &[]);
        execute(deps.as_mut(), env, info, msg).map_err(|e|format!("add asset btc: {}", e))?;
    }
    let usdc_rate = rate(80);
    let usdc_min_rate = rate(10);
    let usdc_optimal_rate = rate(50);
    let usdc_max_rate = rate(100);
    {
        let msg = ExecuteMsg::UpdateAsset { 
            denom: usdc, 
            decimals: 6, 
            target_utilization_rate_bps: usdc_rate, 
            min_rate: usdc_min_rate, 
            optimal_rate: usdc_optimal_rate, 
            max_rate: usdc_max_rate, 
        }; 
        let env = mock_env();
        let info = mock_info(&creator, &[]);
        execute(deps.as_mut(), env, info, msg).map_err(|e|format!("add asset usdc: {}", e))?;
    }
    {
        let msg = ExecuteMsg::DepositCollateral {};
        let env = mock_env();
        let info = mock_info(&alice, &[
            Coin { denom: btc.clone(), amount: Uint128::new(100)},
        ]);
        execute(deps.as_mut(), env, info, msg).map_err(|e|format!("deposit collateral: {}", e))?;
    }


    Ok(())
}   