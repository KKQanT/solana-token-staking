use anchor_lang::prelude::*;

#[account]
pub struct PoolAccount {
    pub owner: Pubkey, //32
    pub token_address : Pubkey, //32
    pub reward_normal_rate_pct: u8, //1
    pub lock_duration: i64, //8
    pub reward_extra_rate_pct: u8, //1
}

impl PoolAccount {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 8 + 1 ;
}
