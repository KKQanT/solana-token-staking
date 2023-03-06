use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod util;

pub use instructions::*;
pub use state::*;
pub use errors::*;
pub use util::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token_staking {
    use super::*;

    pub fn initialize_stake_pool(
        ctx: Context<InitializeStakePool>,
        token_address: Pubkey,
        reward_normal_rate_pct: u8,
        lock_duration: i64,
        reward_extra_rate_pct: u8
    ) -> Result<()> {
        instructions::initialize_stake_pool::handler(
            ctx, 
            token_address,
            reward_normal_rate_pct,
            lock_duration,
            reward_extra_rate_pct
        )
    }
}

#[derive(Accounts)]
pub struct Initialize {}
