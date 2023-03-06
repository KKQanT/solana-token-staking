use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError{
    #[msg("token not with pool account")]
    InvalidToken
}