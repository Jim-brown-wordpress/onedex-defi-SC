use crate::{contract_setup::OneDexContractSetup, to_managed_biguint};
use onedex_sc::{
    logic::pair::PairLogicModule
};

use multiversx_sc_scenario::{
    managed_token_id, num_bigint, rust_biguint, DebugApi,
};

use multiversx_sc::{
    types::{
        Address,
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
    pub fn create_pair(
        &mut self,
        pair_owner: &Address,
        first_token_id: &[u8],
        second_token_id: &[u8],
    ) {
        let big_zero = rust_biguint!(0u64);

        self.blockchain_wrapper
            .execute_tx(pair_owner, &self.onedex_sc_wrapper, &big_zero, |sc| {
                let first_token_id = managed_token_id!(first_token_id);
                let second_token_id = managed_token_id!(second_token_id);

                sc.create_pair(
                    first_token_id,
                    second_token_id,
                )
            })
            .assert_ok();
    }

    pub fn enable_swap(
        &mut self,
        pair_owner: &Address,
        pair_id: usize,
        egld_amount: num_bigint::BigUint
    ) {
        self.blockchain_wrapper
            .execute_tx(pair_owner, &self.onedex_sc_wrapper, &egld_amount, |sc| {
                sc.enable_swap(
                    pair_id
                )
            })
            .assert_ok();

        self.check_user_egld_balance(
            &self.treasury,
            &(egld_amount.clone() / &rust_biguint!(2))
        );

        self.check_user_egld_balance(
            &self.burner,
            &(egld_amount.clone() / &rust_biguint!(2))
        );
    }    
}