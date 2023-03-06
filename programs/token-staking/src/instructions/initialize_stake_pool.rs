use anchor_lang::prelude::*;
use crate::state::{PoolAccount};
use crate::util::print_pool_account_info;

#[derive(Accounts)]
#[instruction(token_address: Pubkey)]
pub struct  InitializeStakePool<'info> {
  #[account(
      init,
      seeds=[
        b"pool", 
        pool_owner.key().as_ref(), 
        token_address.as_ref()
        ],
      bump,
      payer = pool_owner,
      space = PoolAccount::LEN,
  )]
  pub pool_account: Account<'info, PoolAccount>,
  
  #[account(mut)]
  pub pool_owner: Signer<'info>,

  pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeStakePool>, 
    token_address:Pubkey,
    reward_normal_rate_pct:u8,
    lock_duration: i64,
    reward_extra_rate_pct: u8,
) -> Result<()> {
  msg!("creating stake pool");
  let pool_account = &mut ctx.accounts.pool_account;
  pool_account.owner = ctx.accounts.pool_owner.key();
  pool_account.token_address = token_address;
  pool_account.reward_normal_rate_pct = reward_normal_rate_pct;
  pool_account.lock_duration = lock_duration;
  pool_account.reward_extra_rate_pct = reward_extra_rate_pct;

  print_pool_account_info(pool_account);

  Ok(())
}

  