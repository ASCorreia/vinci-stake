use crate::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct StakeCtx<'info>{
    //TBD Validate StakeEntry and StakePool seed through anchor macros
    #[account(mut, constraint = stake_entry.pool == stake_pool.key() @ CustomError::InvalidStakePool)]
    pub stake_entry: Account<'info, StakeEntry>,
    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,

    #[account(mut)]
    pub from_mint_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_mint_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}