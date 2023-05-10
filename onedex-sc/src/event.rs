multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait EventModule {
    #[event("CreatePairEvent")]
    fn create_pair_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] first_token_id: TokenIdentifier,
        #[indexed] second_token_id: TokenIdentifier,
        #[indexed] timestamp: u64,
    );


    #[event("IssueLpEvent")]
    fn issue_lp_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_ticker: &ManagedBuffer,
    );


    #[event("IssueLpSuccessEvent")]
    fn issue_lp_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
    );


    #[event("IssueLpFailureEvent")]
    fn issue_lp_failure_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] message: &ManagedBuffer,
    );


    #[event("AddInitialLiquidityEvent")]
    fn add_initial_liquidity_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] first_token_id: TokenIdentifier,
        #[indexed] first_token_amount: BigUint,
        #[indexed] second_token_id: TokenIdentifier,
        #[indexed] second_token_amount: BigUint,
        #[indexed] lp_token_id: TokenIdentifier,
        #[indexed] lp_token_add_amount: BigUint,
        #[indexed] timestamp: u64,
    );


    #[event("AddLiquidityEvent")]
    fn add_liquidity_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] first_token_id: TokenIdentifier,
        #[indexed] first_token_amount: BigUint,
        #[indexed] second_token_id: TokenIdentifier,
        #[indexed] second_token_amount: BigUint,
        #[indexed] lp_token_id: TokenIdentifier,
        #[indexed] lp_token_add_amount: BigUint,
        #[indexed] timestamp: u64,
    );


    #[event("RemoveLiquidityEvent")]
    fn remove_liquidity_event(
        &self,
        #[indexed] caller: ManagedAddress,
        #[indexed] first_token_id: TokenIdentifier,
        #[indexed] first_token_withdraw_amount: BigUint,
        #[indexed] second_token_id: TokenIdentifier,
        #[indexed] second_token_withdraw_amount: BigUint,
        #[indexed] lp_token_id: TokenIdentifier,
        #[indexed] lp_token_remove_amount: BigUint,
        #[indexed] timestamp: u64,
    );


    #[event("SwapTokensFixedInputEvent")]
    fn swap_tokens_fixed_input_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_in: &TokenIdentifier,
        #[indexed] amount_in: &BigUint,
        #[indexed] token_out: &TokenIdentifier,
        #[indexed] amount_out: &BigUint,
        #[indexed] timestamp: u64,
    );


    #[event("SwapTokensFixedOutputEvent")]
    fn swap_tokens_fixed_output_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_in: &TokenIdentifier,
        #[indexed] amount_in: &BigUint,
        #[indexed] token_out: &TokenIdentifier,
        #[indexed] amount_out: &BigUint,
        #[indexed] timestamp: u64,
    );


    #[event("SwapMultiTokensFixedInputEvent")]
    fn swap_multi_tokens_fixed_input_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_in: &TokenIdentifier,
        #[indexed] amount_in: &BigUint,
        #[indexed] token_out: &TokenIdentifier,
        #[indexed] amount_out: &BigUint,
        #[indexed] timestamp: u64,
    );


    #[event("SwapMultiTokensFixedOutputEvent")]
    fn swap_multi_tokens_fixed_output_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_in: &TokenIdentifier,
        #[indexed] amount_in: &BigUint,
        #[indexed] token_out: &TokenIdentifier,
        #[indexed] amount_out: &BigUint,
        #[indexed] timestamp: u64,
    );


    #[event("TestEvent")]
    fn test_event(
        &self,
        #[indexed] data: &ManagedAddress,
    );
}