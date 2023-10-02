use vinci_accounts::{program::VinciAccounts, BaseAccount};

use crate::*;

#[account]
pub struct StakeEntry {
    pub pool: Pubkey,
    pub amount: u64,
    pub original_mint: Pubkey,
    pub last_staked_at: i64,
    pub total_stake_seconds: u128,
    pub original_mint_seconds_struct: Vec<StakeTime>, //To be discussed as an approach to store mint time (if only one stake entry is used per user)
    pub cooldown_start_seconds: Option<i64>,
    pub last_updated_at: Option<i64>,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub stake_entry: AccountInfo<'info>,

    #[account(mut)]
    pub vinci_account: Box<Account<'info, BaseAccount>>,
    #[account(mut)]
    pub owner: Signer<'info>,

    pub accounts_program: Program<'info, VinciAccounts>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StakeTime {
    pub time: u128,
    pub mint: Pubkey,
}