use crate::{contract_setup::OneDexContractSetup, to_managed_biguint};
use onedex_sc::{
    logic::swap::{SwapLogicModule, self}
};

use multiversx_sc_scenario::{
    managed_token_id, num_bigint, rust_biguint, DebugApi,
};

use multiversx_sc::{
    types::{
        Address, MultiValueEncoded,
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
    pub fn swap_fixed_input(
        &mut self,
        user: &Address,
        in_token_id: &[u8],
        amount: num_bigint::BigUint,
        amount_out_min: num_bigint::BigUint,
        unwrap_required: bool,
        out_token_id:  &[u8],
    ) {

        self.blockchain_wrapper
            .execute_esdt_transfer(user, &self.onedex_sc_wrapper, in_token_id, 0, &amount, |sc| {
                let mut swap_path = MultiValueEncoded::new();

                swap_path.push(
                    managed_token_id!(in_token_id)
                );

                swap_path.push(
                    managed_token_id!(out_token_id)
                );

                sc.swap_multi_tokens_fixed_input(
                    to_managed_biguint(amount_out_min),
                    unwrap_required,
                    swap_path
                )
            })
            .assert_ok();
    }

    pub fn swap_fixed_output(
        &mut self,
        user: &Address,
        amount: num_bigint::BigUint,
        amount_out: num_bigint::BigUint,
        unwrap_required: bool,
        in_token_id: &[u8],
        out_token_id:  &[u8],
    ) {

        self.blockchain_wrapper
            .execute_esdt_transfer(user, &self.onedex_sc_wrapper, in_token_id, 0, &amount, |sc| {
                let mut swap_path = MultiValueEncoded::new();

                swap_path.push(
                    managed_token_id!(in_token_id)
                );

                swap_path.push(
                    managed_token_id!(out_token_id)
                );

                sc.swap_multi_tokens_fixed_output(
                    to_managed_biguint(amount_out),
                    unwrap_required,
                    swap_path
                )
            })
            .assert_ok();
    }
}