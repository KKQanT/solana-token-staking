use anchor_lang::prelude::*;

#[account]
pub struct VaultAccount {
    pub vault_id: Pubkey, // 32
    pub owner: Pubkey, // 32
    pub pool: Pubkey, // 32
    pub staked_amount: u64, // 8
    pub stake_time: i64, //8
    pub unlock_time: i64, //8
    pub last_claimed_time: i64, //8
    pub use_nft: bool, //1
}

impl VaultAccount {
  pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 1;
}
