multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait CommonStorageModule {
    /**
     * WEGLD TokenIdentifier
     */
    #[view(getWegldTokenId)]
    #[storage_mapper("wegld_token_id")]
    fn wegld_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    /**
     * USDC TokenIdentifier
     */
    #[view(getUsdcTokenId)]
    #[storage_mapper("usdc_token_id")]
    fn usdc_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    /**
     * BUSD TokenIdentifier
     */
    #[view(getBusdTokenId)]
    #[storage_mapper("busd_token_id")]
    fn busd_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    /**
     * USDT TokenIdentifier
     */
    #[view(getUsdtTokenId)]
    #[storage_mapper("usdt_token_id")]
    fn usdt_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    /**
     * Total Fee Percentage
     *  Service Fee Percentage + Liquidity Provider Reward Fee Percentage
     *  Decimal: 2
     *  30 -> 0.3%
     */
    #[view(getTotalFeePercent)]
    #[storage_mapper("total_fee_percent")]
    fn total_fee_percent(&self) -> SingleValueMapper<u64>;

    /**
     * Service Fee
     *  Dex Team Commission
     *  Decimal: 2
     *  5 -> 0.05%
     */
    #[view(getSpecialFeePercent)]
    #[storage_mapper("special_fee_percent")]
    fn special_fee_percent(&self) -> SingleValueMapper<u64>;

    #[view(getStakingRewardFeePercent)]
    #[storage_mapper("staking_reward_fee_percent")]
    fn staking_reward_fee_percent(&self) -> SingleValueMapper<u64>;

    /**
     * Treasury Address
     *  Address which Service fee will be transferred
     */
    #[view(getTreasuryAddreess)]
    #[storage_mapper("treasury_address")]
    fn treasury_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getStakingRewardAddress)]
    #[storage_mapper("staking_reward_address")]
    fn staking_reward_address(&self) -> SingleValueMapper<ManagedAddress>;


    #[view(getBurnerAddreess)]
    #[storage_mapper("burner_address")]
    fn burner_address(&self) -> SingleValueMapper<ManagedAddress>;


    /**
     * Unwrap Address
     *  Shard 1 WrappedEgld SC Address
     */
    #[view(getUnwrapAddreess)]
    #[storage_mapper("unwrap_address")]
    fn unwrap_address(&self) -> SingleValueMapper<ManagedAddress>;


    /**
     * Registering Cost
     *  cost: 2 EGLD
     */
    #[view(getRegisteringCost)]
    #[storage_mapper("registering_cost")]
    fn registering_cost(&self) -> SingleValueMapper<BigUint>;

    /**
     * Paused
     *  true: not allowed to operate against sc
     *  false: allowed
     */
    #[view(getPaused)]
    #[storage_mapper("paused")]
    fn paused(&self) -> SingleValueMapper<bool>;

    /**
     * Pair Ids
     *  (first_token_id, second_token_id) -> pair_id
     */
    #[view(getPairIds)]
    #[storage_mapper("pair_ids")]
    fn pair_ids(&self) -> MapMapper<(TokenIdentifier, TokenIdentifier), usize>;

    /**
     * Last Pair Id
     */
    #[view(getLastPairId)]
    #[storage_mapper("last_pair_id")]
    fn last_pair_id(&self) -> SingleValueMapper<usize>;

    /**
     * Lp token_id -> pair_id
     */
    #[view(getLpTokenPairIdMap)]
    #[storage_mapper("lp_token_pair_id_map")]
    fn lp_token_pair_id_map(&self) -> MapMapper<TokenIdentifier, usize>;
}