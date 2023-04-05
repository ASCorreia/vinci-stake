use crate::*;

#[derive(Accounts)]
pub struct InitializeStakeEntry<'info> {
    #[account(init, seeds = [b"VinciWorldStakeEntry_16", user.key().as_ref()], bump, payer = user, space = 3500)] //stake_pool_account.key().as_ref()
    pub stake_entry: Account <'info, StakeEntry>,
    #[account(mut)]
    pub stake_pool_account: Account<'info, StakePool>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub original_mint: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub original_mint_metadata: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeEntry {
    pub pool: Pubkey,
    pub amount: u64,
    pub original_mint: Pubkey,
    pub original_mint_claimed: bool,
    pub last_staker: Pubkey,
    pub last_staked_at: i64,
    pub total_stake_seconds: u128,
    pub stake_mint_claimed: bool,
    pub stake_mint: Option<Pubkey>,
    pub cooldown_start_seconds: Option<i64>,
    pub last_updated_at: Option<i64>,
    pub original_owner: Pubkey,
    pub staking_owner: Pubkey,
}