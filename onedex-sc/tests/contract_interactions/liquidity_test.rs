use crate::{contract_setup::OneDexContractSetup, to_managed_biguint};
use onedex_sc::{
    logic::liquidity::LiquidityLogicModule
};

use multiversx_sc_scenario::{
    managed_token_id, num_bigint, rust_biguint, DebugApi, testing_framework::TxTokenTransfer,
};

use multiversx_sc::{
    types::{
        Address,
        BigUint
    },
    codec::{
        multi_types:: {
            OptionalValue
        }
    },

};

impl<OneDexContractObjBuilder, WegldSwapContractObjBuilder> OneDexContractSetup<OneDexContractObjBuilder, WegldSwapContractObjBuilder>
where
    OneDexContractObjBuilder: 'static + Copy + Fn() -> onedex_sc::ContractObj<DebugApi>,
    WegldSwapContractObjBuilder: 'static + Copy + Fn() -> multiversx_wegld_swap_sc::ContractObj<DebugApi>,
{
    pub fn add_initial_liquidity(
        &mut self,
        pair_owner: &Address,
        first_token_id: &[u8],
        second_token_id: &[u8],
        first_token_amount: num_bigint::BigUint,
        second_token_amount: num_bigint::BigUint
    ) {
        self.blockchain_wrapper
            .execute_esdt_multi_transfer(
                pair_owner,
                &self.onedex_sc_wrapper,
                &vec![
                    TxTokenTransfer {
                        token_identifier: first_token_id.to_vec(),
                        nonce: 0,
                        value: first_token_amount
                    },
                    TxTokenTransfer {
                        token_identifier: second_token_id.to_vec(),
                        nonce: 0,
                        value: second_token_amount
                    },
                ],
                |sc| {
                    sc.add_initial_liquidity();
                },
            )
            .assert_ok();
    }

    pub fn add_liquidity(
        &mut self,
        user: &Address,
        first_token_id: &[u8],
        second_token_id: &[u8],
        first_token_amount: num_bigint::BigUint,
        second_token_amount: num_bigint::BigUint
    ) {
        self.blockchain_wrapper
            .execute_esdt_multi_transfer(
                user,
                &self.onedex_sc_wrapper,
                &vec![
                    TxTokenTransfer {
                        token_identifier: first_token_id.to_vec(),
                        nonce: 0,
                        value: first_token_amount
                    },
                    TxTokenTransfer {
                        token_identifier: second_token_id.to_vec(),
                        nonce: 0,
                        value: second_token_amount
                    },
                ],
                |sc| {
                    sc.add_liquidity(
                        BigUint::from(1u64),
                        BigUint::from(1u64)
                    );
                },
            )
            .assert_ok();
    }

    pub fn remove_liquidity(
        &mut self,
        user: &Address,
        lp_token_id: &[u8],
        lp_token_amount: num_bigint::BigUint,
    ) {
        self.blockchain_wrapper
            .execute_esdt_transfer(
                user,
                &self.onedex_sc_wrapper,
                lp_token_id,
                0,
                &lp_token_amount,
                |sc| {
                    sc.remove_liquidity(
                        BigUint::from(1u64),
                        BigUint::from(1u64),
                        true
                    );
                },
            )
            .assert_ok();
    }
}