#![no_std]

use crate::constants::TOTAL_PERCENT;

multiversx_sc::imports!();

pub mod storage;
pub mod logic;
mod state;
mod event;
mod constants;
mod view;
mod proxy;

#[multiversx_sc::contract]
pub trait OneDex:
    storage::common_storage::CommonStorageModule
    + storage::pair_storage::PairStorageModule
    + logic::common::CommonLogicModule
    + logic::pair::PairLogicModule
    + logic::liquidity::LiquidityLogicModule
    + logic::swap::SwapLogicModule
    + logic::amm::AmmLogicModule
    + event::EventModule
    + view::ViewModule
{
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(setConfig)]
    fn set_config(
        &self,
        wegld_token_id: TokenIdentifier,
        usdc_token_id: TokenIdentifier,
        busd_token_id: TokenIdentifier,
        usdt_token_id: TokenIdentifier,

        total_fee_percent: u64,
        special_fee_percent: u64,
        staking_reward_fee_percent: u64,

        treasury_address: ManagedAddress,
        staking_reward_address: ManagedAddress,
        burner_address: ManagedAddress,
        unwrap_address: ManagedAddress,

        registering_cost: BigUint
    ) {
        self.wegld_token_id().set(&wegld_token_id);
        self.usdc_token_id().set(&usdc_token_id);
        self.busd_token_id().set(&busd_token_id);
        self.usdt_token_id().set(&usdt_token_id);

        self.set_total_fee_percent(total_fee_percent);
        self.set_special_fee_percent(special_fee_percent);
        self.set_staking_reward_fee_percent(staking_reward_fee_percent);
        
        self.set_treasury_address(treasury_address);
        self.set_staking_reward_address(staking_reward_address);
        self.set_burner_address(burner_address);
        self.set_unwrap_address(unwrap_address);
        
        self.set_registering_cost(registering_cost);
    }


    #[only_owner]
    #[endpoint(setTotalFeePercent)]
    fn set_total_fee_percent(
        &self,
        total_fee_percent: u64,
    ) {
        require!(
            total_fee_percent <= TOTAL_PERCENT,
            "invalid total fee percent"
        );

        require!(
            self.staking_reward_fee_percent().get() + self.special_fee_percent().get() <= total_fee_percent,
            "invalid total fee percent"
        );

        self.total_fee_percent().set(total_fee_percent);
    }


    #[only_owner]
    #[endpoint(setSpecialFeePercent)]
    fn set_special_fee_percent(
        &self,
        special_fee_percent: u64,
    ) {
        require!(
            special_fee_percent + self.staking_reward_fee_percent().get() <= self.total_fee_percent().get(),
            "invalid special fee percent"
        );

        self.special_fee_percent().set(special_fee_percent);
    }

    #[only_owner]
    #[endpoint(setStakingRewardFeePercent)]
    fn set_staking_reward_fee_percent(
        &self,
        staking_reward_fee_percent: u64
    ) {
        require!(
            staking_reward_fee_percent + self.special_fee_percent().get() <= self.total_fee_percent().get(),
            "invalid staking reward fee percent"
        );

        self.staking_reward_fee_percent().set(staking_reward_fee_percent);
    }

    #[only_owner]
    #[endpoint(setStakingRewardAddress)]
    fn set_staking_reward_address(
        &self,
        staking_reward_address: ManagedAddress
    ) {
        self.staking_reward_address().set(&staking_reward_address);
    }

    #[only_owner]
    #[endpoint(setTreasuryAddress)]
    fn set_treasury_address(
        &self,
        treasury_address: ManagedAddress,
    ) {
        self.treasury_address().set(&treasury_address);
    }

    #[only_owner]
    #[endpoint(setBurnerAddress)]
    fn set_burner_address(
        &self,
        burner_address: ManagedAddress,
    ) {
        self.burner_address().set(&burner_address);
    }

    #[only_owner]
    #[endpoint(setUnwrapAddress)]
    fn set_unwrap_address(
        &self,
        unwrap_address: ManagedAddress,
    ) {
        self.unwrap_address().set(&unwrap_address);
    }

    #[only_owner]
    #[endpoint(setRegisteringCost)]
    fn set_registering_cost(
        &self,
        registering_cost: BigUint,
    ) {
        self.registering_cost().set(&registering_cost);
    }
}
