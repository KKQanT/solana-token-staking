use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};
use crate::errors::{StakingError};
use crate::state::{PoolAccount, VaultAccount};

#[derive(Accounts)]
#[instruction(
  pool_account_owner: Pubkey, 
  vault_id: Pubkey, 
  pool_bump: u8)]
pub struct Stake<'info> {
    #[account(
        init,
        seeds = [
            b"vault", 
            vault_id.as_ref(), 
            pool_account.key().as_ref(), 
            user.key().as_ref()
            ],
        bump,
        payer = user,
        space = VaultAccount::LEN,
    )]
    pub vault_account: Account<'info, VaultAccount>,
    #[account(
        mut,
        seeds = [
            b"pool",
            pool_account_owner.as_ref(),
            user_token_account.mint.as_ref(),
        ],
        bump=pool_bump
    )]
    pub pool_account: Account<'info, PoolAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub vault_token_account: Account<'info, token::TokenAccount>, /// checked in handler
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}

pub fn handler(
    ctx: Context<Stake>, 
    _pool_account_owner: Pubkey, 
    vault_id: Pubkey,
    _pool_bump: u8,
    staked_amount: u64,
    lock_duration: i64,
  ) -> Result<()> {

    let stake_time = Clock::get().unwrap().unix_timestamp;
    let unlock_time = stake_time + lock_duration;

    let pool_account = & ctx.accounts.pool_account;
    let user_token_account = & ctx.accounts.user_token_account;

    if user_token_account.mint != pool_account.token_address {
        return err!(StakingError::InvalidToken)
    }

    let vault_account = &mut ctx.accounts.vault_account;

    vault_account.vault_id = vault_id;
    vault_account.owner = ctx.accounts.user.key();
    vault_account.pool = ctx.accounts.pool_account.key();
    vault_account.staked_amount = staked_amount;
    vault_account.stake_time = stake_time.clone();
    vault_account.unlock_time = unlock_time;
    vault_account.last_claimed_time = stake_time.clone();
    vault_account.use_nft = false;

    Ok(())
  }
  