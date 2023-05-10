use onedex_sc:: {
    storage::pair_storage::PairStorageModule
};

use crate::{contract_setup::OneDexContractSetup, to_managed_biguint };
use multiversx_sc::{
    types::{
        Address,
        BigUint
    }
};

use multiversx_sc_scenario::{
    rust_biguint, DebugApi, managed_address, managed_token_id
};

/**
 * Common Utils
 */
impl<OneDexContractObjBuilder, WegldSwapContractObjBuilder> OneDexContractSetup<OneDexContractObjBuilder, WegldSwapContractObjBuilder>
where
    OneDexContractObjBuilder: 'static + Copy + Fn() -> onedex_sc::ContractObj<DebugApi>,
    WegldSwapContractObjBuilder: 'static + Copy + Fn() -> multiversx_wegld_swap_sc::ContractObj<DebugApi>,
{
    pub fn setup_new_user(
        &mut self,
        egld_mount: u64
    ) -> Address {
        let big_zero = rust_biguint!(0);

        let new_user = self.blockchain_wrapper.create_user_account(&big_zero);
        
        self.blockchain_wrapper
            .set_egld_balance(&new_user, &Self::exp18(egld_mount));
        
        new_user
    }

    pub fn set_esdt_balance(&mut self, address: &Address, token_id: &[u8], balance: &num_bigint::BigUint) {
        self.blockchain_wrapper
            .set_esdt_balance(address, token_id, balance);
    }

    pub fn set_egld_balance(&mut self, address: &Address, balance: &num_bigint::BigUint) {
        self.blockchain_wrapper
            .set_egld_balance(address, balance);
    }

    pub fn check_user_balance(&self, address: &Address, token_id: &[u8], token_balance: &num_bigint::BigUint) {
        self.blockchain_wrapper
            .check_esdt_balance(address, token_id, token_balance);
    }

    pub fn check_user_egld_balance(&self, address: &Address, token_balance: &num_bigint::BigUint) {
        self.blockchain_wrapper
            .check_egld_balance(address, token_balance);
    }

    pub fn check_pool_state(
        &mut self,
        pair_id: usize,
        pair_owner: &Address,
        first_token_id: &[u8],
        second_token_id: &[u8],
        first_token_reserve: num_bigint::BigUint,
        second_token_reserve: num_bigint::BigUint,
        lp_token_supply: num_bigint::BigUint,
    ) {
        self.blockchain_wrapper
            .execute_query(
                &self.onedex_sc_wrapper, |sc| {
                    assert_eq!(
                        sc.pair_owner(pair_id).get(),
                        managed_address!(pair_owner)
                    );

                    assert_eq!(
                        sc.pair_first_token_id(pair_id).get(),
                        managed_token_id!(first_token_id)
                    );

                    assert_eq!(
                        sc.pair_second_token_id(pair_id).get(),
                        managed_token_id!(second_token_id)
                    );
                    
                    assert_eq!(
                        sc.pair_first_token_reserve(pair_id).get(),
                        to_managed_biguint(first_token_reserve)
                    );

                    assert_eq!(
                        sc.pair_second_token_reserve(pair_id).get(),
                        to_managed_biguint(second_token_reserve)
                    );

                    assert_eq!(
                        sc.pair_lp_token_supply(pair_id).get(),
                        to_managed_biguint(lp_token_supply)
                    );
                }
            ).assert_ok();
    }
}