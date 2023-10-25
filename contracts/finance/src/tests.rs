use cosmwasm_std::{testing::{mock_env, mock_info, mock_dependencies}, Addr, Coin, Uint128};

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
    let usdc: String = Addr::unchecked("usdc").into();
    // let alice_balances = [
    //     Coin { denom: btc.clone(), amount: Uint128::new(10_000), },
    //     Coin { denom: usdc.clone(), amount: Uint128::new(100_000), },
    // ];
    // let bob_balances = [
    //     Coin { denom: btc.clone(), amount: Uint128::new(10_000), },
    //     Coin { denom: usdc.clone(), amount: Uint128::new(100_000), },
    // ];
    // let balances = [
    //     (alice.as_str(), alice_balances.as_slice()),
    // ];
    let mut deps = mock_dependencies();
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
            denom: btc.clone(), 
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
            denom: usdc.clone(), 
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
            Coin { denom: btc.clone(), amount: Uint128::new(2)}
        ]);
        execute(deps.as_mut(), env, info, msg).map_err(|e|format!("deposit collateral: {}", e))?;
    }
    {
        let msg = ExecuteMsg::WithdrawCollateral { 
            denom: btc.clone(), 
            amount: Uint128::new(1), 
        };
        let env = mock_env();
        let info = mock_info(&alice, &[]);
        execute(deps.as_mut(), env, info, msg).map_err(|e|format!("withdraw collateral: {}", e))?;
    }
    {
        let msg = ExecuteMsg::Deposit {};
        let env = mock_env();
        let info = mock_info(&alice, &[
            Coin { denom: usdc.clone(), amount: Uint128::new(100_000)}
        ]);
        execute(deps.as_mut(), env, info, msg).map_err(|e|format!("deposit: {}", e))?;
    }
    {
        let msg = ExecuteMsg::Borrow { 
            denom: usdc.clone(), 
            amount: Uint128::new(10_000), 
        };
        let env = mock_env();
        let info = mock_info(&alice, &[
            Coin { denom: usdc.clone(), amount: Uint128::new(100_000)}
        ]);
        execute(deps.as_mut(), env, info, msg).map_err(|e|format!("deposit: {}", e))?;
    }

    Ok(())
}   