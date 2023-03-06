use anchor_lang::prelude::msg;
use crate::state::PoolAccount;

pub fn print_pool_account_info(
    pool_account: &PoolAccount
) {
    msg!("stake pool created");
    msg!("owner = {}", pool_account.owner.to_string());
    msg!("token_address = {}", pool_account.token_address.to_string());
    msg!("reward_normal_rate_pct = {}", pool_account.reward_normal_rate_pct);
    msg!("lock_duration = {}", pool_account.lock_duration);
    msg!("reward_extra_rate_pct = {}", pool_account.reward_extra_rate_pct);
}