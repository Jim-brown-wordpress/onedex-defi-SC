use onedex_sc::{*, storage::{pair_storage::PairStorageModule, common_storage::CommonStorageModule}};
use multiversx_wegld_swap_sc::*;

use std::ops::Mul;

use multiversx_sc::{
    types::{
        Address,
        BigUint,
        EsdtLocalRole
    },
};

use multiversx_sc_scenario::{
    managed_address, managed_token_id, num_bigint, rust_biguint, testing_framework::*, DebugApi,
};

use crate::constant::*;

pub static ESDT_ROLES: &[EsdtLocalRole] = &[
    EsdtLocalRole::Mint,
    EsdtLocalRole::Burn,
    EsdtLocalRole::Transfer,
];

pub struct OneDexContractSetup<OneDexContractObjBuilder, WegldSwapContractObjBuilder>
where
    OneDexContractObjBuilder: 'static + Copy + Fn() -> onedex_sc::ContractObj<DebugApi>,
    WegldSwapContractObjBuilder: 'static + Copy + Fn() -> multiversx_wegld_swap_sc::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub treasury: Address,
    pub burner: Address,
    pub wegld_swap_wrapper:
        ContractObjWrapper<multiversx_wegld_swap_sc::ContractObj<DebugApi>, WegldSwapContractObjBuilder>,
    pub onedex_sc_wrapper:
        ContractObjWrapper<onedex_sc::ContractObj<DebugApi>, OneDexContractObjBuilder>
}



impl<OneDexContractObjBuilder, WegldSwapContractObjBuilder> OneDexContractSetup<OneDexContractObjBuilder, WegldSwapContractObjBuilder>
where
    OneDexContractObjBuilder: 'static + Copy + Fn() -> onedex_sc::ContractObj<DebugApi>,
    WegldSwapContractObjBuilder: 'static + Copy + Fn() -> multiversx_wegld_swap_sc::ContractObj<DebugApi>,
{
    pub fn new(onedex_sc_builder: OneDexContractObjBuilder, wegld_swap_builder: WegldSwapContractObjBuilder) -> Self {
        let big_zero = rust_biguint!(0u64);

        let mut blockchain_wrapper = BlockchainStateWrapper::new();
        
        let owner_address = blockchain_wrapper.create_user_account(&big_zero);

        let wegld_swap_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            wegld_swap_builder,
            WEGLD_SWAP_WASM_PATH,
        );

        let onedex_sc_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            onedex_sc_builder,
            ONE_DEX_WASM_PATH,
        );

        
        // wegld swap contract deploy
        blockchain_wrapper
            .execute_tx(&owner_address, &wegld_swap_wrapper, &big_zero, |sc|{
                let wegld_token_id = managed_token_id!(WEGLD_TOKEN_ID);

                sc.init(wegld_token_id);
            })
            .assert_ok();

        // set egld balance and wegld balance on wegld swap sc
        blockchain_wrapper
            .set_egld_balance(&wegld_swap_wrapper.address_ref(),  &Self::exp18(1000));
        blockchain_wrapper
            .set_esdt_balance(&wegld_swap_wrapper.address_ref(), WEGLD_TOKEN_ID, &Self::exp18(1000));

        // onedex contract deploy
        blockchain_wrapper
            .execute_tx(&owner_address, &onedex_sc_wrapper, &big_zero, |sc|{
                sc.init();
            })
            .assert_ok();

        // config value
        let treasury = blockchain_wrapper.create_user_account(&big_zero);
        let burner = blockchain_wrapper.create_user_account(&big_zero);
        
        // onedex contract set config
        blockchain_wrapper
            .execute_tx(&owner_address, &onedex_sc_wrapper, &big_zero, |sc| {
                let wegld_token_id = managed_token_id!(WEGLD_TOKEN_ID);
                let usdc_token_id = managed_token_id!(USDC_TOKEN_ID);
                let busd_token_id = managed_token_id!(BUSD_TOKEN_ID);
                let usdt_token_id = managed_token_id!(USDT_TOKEN_ID);

                let treasury_address = managed_address!(&treasury);
                let burner_address = managed_address!(&burner);
                let unwrap_address = managed_address!(wegld_swap_wrapper.address_ref());
                let registering_cost = Self::to_managed_biguint(Self::exp18(REGISTERING_COST));

                sc.set_config(wegld_token_id, usdc_token_id, busd_token_id, usdt_token_id, TOTAL_FEE_PERCENT, SPECIAL_FEE_PERCENT, treasury_address, burner_address, unwrap_address, registering_cost)
            })
            .assert_ok();

        blockchain_wrapper
            .execute_tx(&owner_address, &onedex_sc_wrapper, &big_zero, |sc| {
                sc.pair_lp_token_id(1).set(managed_token_id!(ONE_WEGLD_LP_ID));
                sc.pair_lp_token_id(2).set(managed_token_id!(TWO_WEGLD_LP_ID));
                sc.pair_lp_token_id(3).set(managed_token_id!(ONE_USDC_LP_ID));
                sc.pair_lp_token_id(4).set(managed_token_id!(TWO_USDC_LP_ID));

                sc.lp_token_pair_id_map().insert(managed_token_id!(ONE_WEGLD_LP_ID), 1);
            })
            .assert_ok();

        blockchain_wrapper.set_esdt_local_roles(
            wegld_swap_wrapper.address_ref(),
            WEGLD_TOKEN_ID,
            ESDT_ROLES
        );

        blockchain_wrapper.set_esdt_local_roles(
            onedex_sc_wrapper.address_ref(),
            ONE_WEGLD_LP_ID,
            ESDT_ROLES
        );
        
        blockchain_wrapper.set_esdt_local_roles(
            onedex_sc_wrapper.address_ref(),
            TWO_WEGLD_LP_ID,
            ESDT_ROLES
        );

        blockchain_wrapper.set_esdt_local_roles(
            onedex_sc_wrapper.address_ref(),
            ONE_USDC_LP_ID,
            ESDT_ROLES
        );

        blockchain_wrapper.set_esdt_local_roles(
            onedex_sc_wrapper.address_ref(),
            TWO_USDC_LP_ID,
            ESDT_ROLES
        );

        OneDexContractSetup {
            blockchain_wrapper,
            owner_address,
            treasury,
            burner,
            wegld_swap_wrapper,
            onedex_sc_wrapper,
        }
    }

    pub fn to_managed_biguint(value: num_bigint::BigUint) -> BigUint<DebugApi> {
        BigUint::from_bytes_be(&value.to_bytes_be())
    }

    pub fn exp18(value: u64) -> num_bigint::BigUint {
        value.mul(rust_biguint!(10).pow(18))
    }
}
