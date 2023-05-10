mod contract_setup;
mod contract_interactions;

use contract_setup::*;
use multiversx_sc_scenario::DebugApi;

mod constant;
use constant::*;
use num_traits::FromPrimitive;

use std::ops::Mul;

use multiversx_sc::{
    types::{
        BigUint,
    },
};

use multiversx_sc_scenario::{
    rust_biguint
};


#[test]
fn init_test() {
    let _ = OneDexContractSetup::new(onedex_sc::contract_obj, multiversx_wegld_swap_sc::contract_obj);
}

#[test]
fn create_pair_test() {
    let _ = DebugApi::dummy();

    let mut sc_setup = OneDexContractSetup::new(onedex_sc::contract_obj, multiversx_wegld_swap_sc::contract_obj);

    let pair_owner = sc_setup.setup_new_user(1u64);
    let big_zero = rust_biguint!(0);

    let first_token_id = ONE_TOKEN_ID;
    let second_token_id = WEGLD_TOKEN_ID;

    sc_setup.create_pair(&pair_owner, first_token_id, second_token_id);

    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, big_zero.clone(), big_zero.clone(), big_zero.clone());
}

#[test]
fn enable_pair_test() {
    let _ = DebugApi::dummy();

    let mut sc_setup = OneDexContractSetup::new(onedex_sc::contract_obj, multiversx_wegld_swap_sc::contract_obj);

    let pair_owner = sc_setup.setup_new_user(2u64);
    let big_zero = rust_biguint!(0);

    let first_token_id = ONE_TOKEN_ID;
    let second_token_id = WEGLD_TOKEN_ID;

    sc_setup.create_pair(&pair_owner, first_token_id, second_token_id);

    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, big_zero.clone(), big_zero.clone(), big_zero.clone());

    let enable_cost = exp18(2);
    sc_setup.enable_swap(&pair_owner, 1, enable_cost);
}


#[test]
fn add_initial_liquidity_test() {
    let _ = DebugApi::dummy();

    let mut sc_setup = OneDexContractSetup::new(onedex_sc::contract_obj, multiversx_wegld_swap_sc::contract_obj);

    let pair_owner = sc_setup.setup_new_user(0u64);
    let big_zero = rust_biguint!(0);

    let first_token_id = ONE_TOKEN_ID;
    let second_token_id = WEGLD_TOKEN_ID;

    // create one - wegld pair
    sc_setup.create_pair(&pair_owner, first_token_id, second_token_id);

    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, big_zero.clone(), big_zero.clone(), big_zero.clone());

    let first_token_amount = exp18(100);
    let second_token_amount = exp18(10);

    sc_setup.set_esdt_balance(&pair_owner, first_token_id, &first_token_amount);
    sc_setup.set_esdt_balance(&pair_owner, second_token_id, &second_token_amount);

    // add initial liquidity
    // one amount 100, decimal 18
    // welgd amount 10, decimal 18
    sc_setup.add_initial_liquidity(&pair_owner, first_token_id, second_token_id, first_token_amount.clone(), second_token_amount.clone());

    // check storage
    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, first_token_amount, second_token_amount.clone(), second_token_amount.clone());

    let expected_lp_amount = exp18(10) - rust_biguint!(MINIMUM_LIQUIDITY);

    // check balances
    sc_setup.check_user_balance(&pair_owner, ONE_WEGLD_LP_ID, &expected_lp_amount);
    sc_setup.check_user_balance(&pair_owner, first_token_id, &big_zero);
    sc_setup.check_user_balance(&pair_owner, second_token_id, &big_zero);
}

#[test]
fn add_liquidity_test() {
    let _ = DebugApi::dummy();

    let mut sc_setup = OneDexContractSetup::new(onedex_sc::contract_obj, multiversx_wegld_swap_sc::contract_obj);

    let pair_owner = sc_setup.setup_new_user(1u64);
    let big_zero = rust_biguint!(0);

    let first_token_id = ONE_TOKEN_ID;
    let second_token_id = WEGLD_TOKEN_ID;

    // create one - wegld pair
    sc_setup.create_pair(&pair_owner, first_token_id, second_token_id);

    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, big_zero.clone(), big_zero.clone(), big_zero.clone());

    let first_token_amount = exp18(100);
    let second_token_amount = exp18(10);

    sc_setup.set_esdt_balance(&pair_owner, first_token_id, &first_token_amount);
    sc_setup.set_esdt_balance(&pair_owner, second_token_id, &second_token_amount);

    // add initial liquidity
    // one amount 100, decimal 18
    // welgd amount 10, decimal 18
    sc_setup.add_initial_liquidity(&pair_owner, first_token_id, second_token_id, first_token_amount.clone(), second_token_amount.clone());

    // check storage
    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, first_token_amount.clone(), second_token_amount.clone(), second_token_amount.clone());

    let expected_lp_amount = exp18(10) - rust_biguint!(MINIMUM_LIQUIDITY);

    // check balances
    sc_setup.check_user_balance(&pair_owner, ONE_WEGLD_LP_ID, &expected_lp_amount);
    sc_setup.check_user_balance(&pair_owner, first_token_id, &big_zero);
    sc_setup.check_user_balance(&pair_owner, second_token_id, &big_zero);

    let user = sc_setup.setup_new_user(0u64);

    let user_first_token_amount = exp18(100);
    let user_second_token_amount = exp18(10);

    sc_setup.set_esdt_balance(&user, first_token_id, &user_first_token_amount);
    sc_setup.set_esdt_balance(&user, second_token_id, &user_second_token_amount);

    // add initial liquidity
    // one amount 10, decimal 18
    // welgd amount 1, decimal 18
    sc_setup.add_liquidity(&user, first_token_id, second_token_id, user_first_token_amount.clone(), user_second_token_amount.clone());

    // check storage
    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, first_token_amount.clone() + user_first_token_amount.clone(), second_token_amount.clone() + user_second_token_amount.clone(), second_token_amount.clone() + user_second_token_amount.clone());

    let user_expected_lp_amount = exp18(10);

    // check balances
    sc_setup.check_user_balance(&user, ONE_WEGLD_LP_ID, &user_expected_lp_amount);
    sc_setup.check_user_balance(&user, first_token_id, &big_zero);
    sc_setup.check_user_balance(&user, second_token_id, &big_zero);
}

#[test]
fn remove_liquidity_test() {
    let _ = DebugApi::dummy();

    let mut sc_setup = OneDexContractSetup::new(onedex_sc::contract_obj, multiversx_wegld_swap_sc::contract_obj);

    let pair_owner = sc_setup.setup_new_user(0u64);
    let big_zero = rust_biguint!(0);

    let first_token_id = ONE_TOKEN_ID;
    let second_token_id = WEGLD_TOKEN_ID;

    // create one - wegld pair
    sc_setup.create_pair(&pair_owner, first_token_id, second_token_id);

    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, big_zero.clone(), big_zero.clone(), big_zero.clone());

    let first_token_amount = exp18(100);
    let second_token_amount = exp18(10);

    sc_setup.set_esdt_balance(&pair_owner, first_token_id, &first_token_amount);
    sc_setup.set_esdt_balance(&pair_owner, second_token_id, &second_token_amount);

    // add initial liquidity
    // one amount 100, decimal 18
    // welgd amount 10, decimal 18
    sc_setup.add_initial_liquidity(&pair_owner, first_token_id, second_token_id, first_token_amount.clone(), second_token_amount.clone());

    // check storage
    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, first_token_amount.clone(), second_token_amount.clone(), second_token_amount.clone());

    let expected_lp_amount = exp18(10) - rust_biguint!(MINIMUM_LIQUIDITY);

    // check balances
    sc_setup.check_user_balance(&pair_owner, ONE_WEGLD_LP_ID, &expected_lp_amount);
    sc_setup.check_user_balance(&pair_owner, first_token_id, &big_zero);
    sc_setup.check_user_balance(&pair_owner, second_token_id, &big_zero);

    let user = sc_setup.setup_new_user(0u64);

    let user_first_token_amount = exp18(100);
    let user_second_token_amount = exp18(10);

    sc_setup.set_esdt_balance(&user, first_token_id, &user_first_token_amount);
    sc_setup.set_esdt_balance(&user, second_token_id, &user_second_token_amount);

    // add initial liquidity
    // one amount 100, decimal 18
    // welgd amount 10, decimal 18
    sc_setup.add_liquidity(&user, first_token_id, second_token_id, user_first_token_amount.clone(), user_second_token_amount.clone());

    // check storage
    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, first_token_amount.clone() + user_first_token_amount.clone(), second_token_amount.clone() + user_second_token_amount.clone(), second_token_amount.clone() + user_second_token_amount.clone());

    let user_expected_lp_amount = exp18(10);

    // check balances
    sc_setup.check_user_balance(&user, ONE_WEGLD_LP_ID, &user_expected_lp_amount);
    sc_setup.check_user_balance(&user, first_token_id, &big_zero);
    sc_setup.check_user_balance(&user, second_token_id, &big_zero);

    let remove_lp_amount = exp18(5);

    sc_setup.remove_liquidity(&pair_owner, ONE_WEGLD_LP_ID, remove_lp_amount.clone());

    let expected_first_token = exp18(50);
    let expected_second_token = exp18(5);

    let remain_first_token_reserve = exp18(150);
    let remain_second_token_reserve = exp18(15);
    let remain_lp_supply = exp18(15);

    // check pair storage
    sc_setup.check_pool_state(
        1,
        &pair_owner,
        first_token_id,
        second_token_id,
        remain_first_token_reserve,
        remain_second_token_reserve,
        remain_lp_supply
    );

    // check balances
    sc_setup.check_user_balance(&pair_owner, ONE_WEGLD_LP_ID, &(expected_lp_amount - remove_lp_amount));
    sc_setup.check_user_balance(&pair_owner, first_token_id, &expected_first_token);
    sc_setup.check_user_egld_balance(&pair_owner, &expected_second_token);
}

#[test]
fn swap_fixed_input_test() {
    let _ = DebugApi::dummy();

    let mut sc_setup = OneDexContractSetup::new(onedex_sc::contract_obj, multiversx_wegld_swap_sc::contract_obj);

    let pair_owner = sc_setup.setup_new_user(2u64);
    let big_zero = rust_biguint!(0);

    let first_token_id = ONE_TOKEN_ID;
    let second_token_id = WEGLD_TOKEN_ID;

    // create one - wegld pair
    sc_setup.create_pair(&pair_owner, first_token_id, second_token_id,);

    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, big_zero.clone(), big_zero.clone(), big_zero.clone());

    let first_token_amount = exp18(10000);
    let second_token_amount = exp18(1000);

    sc_setup.set_esdt_balance(&pair_owner, first_token_id, &first_token_amount);
    sc_setup.set_esdt_balance(&pair_owner, second_token_id, &second_token_amount);

    // add initial liquidity
    // one amount 1000, decimal 18
    // welgd amount 100, decimal 18
    sc_setup.add_initial_liquidity(&pair_owner, first_token_id, second_token_id, first_token_amount.clone(), second_token_amount.clone());

    // check storage
    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, first_token_amount.clone(), second_token_amount.clone(), second_token_amount.clone());

    let expected_lp_amount = second_token_amount.clone() - rust_biguint!(MINIMUM_LIQUIDITY);

    // check balances
    sc_setup.check_user_balance(&pair_owner, ONE_WEGLD_LP_ID, &expected_lp_amount);
    sc_setup.check_user_balance(&pair_owner, first_token_id, &big_zero);
    sc_setup.check_user_balance(&pair_owner, second_token_id, &big_zero);

    // enable swap
    let enable_cost = exp18(2);
    sc_setup.enable_swap(&pair_owner, 1, enable_cost);

    let user = sc_setup.setup_new_user(0u64);

    let user_first_token_amount = exp18(10000);
    let user_second_token_amount = exp18(1000);

    sc_setup.set_esdt_balance(&user, first_token_id, &user_first_token_amount);
    sc_setup.set_esdt_balance(&user, second_token_id, &user_second_token_amount);

    // add initial liquidity
    // one amount 1000, decimal 18
    // welgd amount 100, decimal 18
    sc_setup.add_liquidity(&user, first_token_id, second_token_id, user_first_token_amount.clone(), user_second_token_amount.clone());

    // check storage
    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, first_token_amount.clone() + user_first_token_amount.clone(), second_token_amount.clone() + user_second_token_amount.clone(), second_token_amount.clone() + user_second_token_amount.clone());

    let user_expected_lp_amount = user_second_token_amount.clone();

    // check balances
    sc_setup.check_user_balance(&user, ONE_WEGLD_LP_ID, &user_expected_lp_amount);
    sc_setup.check_user_balance(&user, first_token_id, &big_zero);
    sc_setup.check_user_balance(&user, second_token_id, &big_zero);

    // start swap
    let swap_user = sc_setup.setup_new_user(0u64);

    // current pool amount one 20000, wegld 2000
    // swap 2000
    let swap_amount = exp18(20000);

    sc_setup.set_esdt_balance(&swap_user, first_token_id, &swap_amount);

    sc_setup.swap_fixed_input(&swap_user, first_token_id, swap_amount.clone(), rust_biguint!(1), true, second_token_id);

    let expected_second_token = exp18(997);

    //
    sc_setup.check_user_egld_balance(&swap_user, &expected_second_token);
}

#[test]
fn swap_fixed_output_test() {
    let _ = DebugApi::dummy();

    let mut sc_setup = OneDexContractSetup::new(onedex_sc::contract_obj, multiversx_wegld_swap_sc::contract_obj);

    let pair_owner = sc_setup.setup_new_user(2u64);
    let big_zero = rust_biguint!(0);

    let first_token_id = ONE_TOKEN_ID;
    let second_token_id = WEGLD_TOKEN_ID;

    // create one - wegld pair
    sc_setup.create_pair(&pair_owner, first_token_id, second_token_id);

    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, big_zero.clone(), big_zero.clone(), big_zero.clone());

    let first_token_amount = exp18(10000);
    let second_token_amount = exp18(1000);

    sc_setup.set_esdt_balance(&pair_owner, first_token_id, &first_token_amount);
    sc_setup.set_esdt_balance(&pair_owner, second_token_id, &second_token_amount);

    // add initial liquidity
    // one amount 1000, decimal 18
    // welgd amount 100, decimal 18
    sc_setup.add_initial_liquidity(&pair_owner, first_token_id, second_token_id, first_token_amount.clone(), second_token_amount.clone());

    // check storage
    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, first_token_amount.clone(), second_token_amount.clone(), second_token_amount.clone());

    let expected_lp_amount = second_token_amount.clone() - rust_biguint!(MINIMUM_LIQUIDITY);

    // check balances
    sc_setup.check_user_balance(&pair_owner, ONE_WEGLD_LP_ID, &expected_lp_amount);
    sc_setup.check_user_balance(&pair_owner, first_token_id, &big_zero);
    sc_setup.check_user_balance(&pair_owner, second_token_id, &big_zero);

    // enable swap
    let enable_cost = exp18(2);
    sc_setup.enable_swap(&pair_owner, 1, enable_cost);

    let user = sc_setup.setup_new_user(0u64);

    let user_first_token_amount = exp18(10000);
    let user_second_token_amount = exp18(1000);

    sc_setup.set_esdt_balance(&user, first_token_id, &user_first_token_amount);
    sc_setup.set_esdt_balance(&user, second_token_id, &user_second_token_amount);

    // add initial liquidity
    // one amount 1000, decimal 18
    // welgd amount 100, decimal 18
    sc_setup.add_liquidity(&user, first_token_id, second_token_id, user_first_token_amount.clone(), user_second_token_amount.clone());

    // check storage
    sc_setup.check_pool_state(1, &pair_owner, first_token_id, second_token_id, first_token_amount.clone() + user_first_token_amount.clone(), second_token_amount.clone() + user_second_token_amount.clone(), second_token_amount.clone() + user_second_token_amount.clone());

    let user_expected_lp_amount = user_second_token_amount.clone();

    // check balances
    sc_setup.check_user_balance(&user, ONE_WEGLD_LP_ID, &user_expected_lp_amount);
    sc_setup.check_user_balance(&user, first_token_id, &big_zero);
    sc_setup.check_user_balance(&user, second_token_id, &big_zero);

    // start swap
    let swap_user = sc_setup.setup_new_user(0u64);

    // current pool amount one 20000, wegld 2000
    // swap 2000
    let swap_amount = exp18(20000) + rust_biguint!(1);
    let swap_wanted_out_amount = exp18(997);

    sc_setup.set_esdt_balance(&swap_user, first_token_id, &swap_amount);

    sc_setup.swap_fixed_output(&swap_user,  swap_amount.clone(), swap_wanted_out_amount.clone(), false, first_token_id, second_token_id);

    //
    sc_setup.check_user_balance(&swap_user, first_token_id, &big_zero);
    sc_setup.check_user_balance(&swap_user, second_token_id, &swap_wanted_out_amount);
}

pub fn exp18(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(18))
}

pub fn to_managed_biguint(value: num_bigint::BigUint) -> BigUint<DebugApi> {
    BigUint::from_bytes_be(&value.to_bytes_be())
}