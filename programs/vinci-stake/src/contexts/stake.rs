use crate::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct StakeCtx<'info>{
    //TBD Validate StakeEntry and StakePool seed through anchor macros
    #[account(mut, constraint = stake_entry.pool == stake_pool.key() @ CustomError::InvalidStakePool)]
    pub stake_entry: Box<Account<'info, StakeEntry>>,
    #[account(mut)]
    pub stake_pool: Box<Account<'info, StakePool>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub original_mint: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,

    #[account(mut)]
    pub from_mint_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to_mint_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub test: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateStakeCtx<'info>{
    pub stake_pool: Signer<'info>,
}