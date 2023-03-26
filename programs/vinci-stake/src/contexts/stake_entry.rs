use crate::*;

#[derive(Accounts)]
pub struct InitializeStakeEntry<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, seeds = [b"VinciWorldStakeEntry", user.key().as_ref(), stake_pool.key().as_ref()], bump, payer = user, space = 3500)]
    pub stake_entry: Account <'info, StakeEntry>,
    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,

    #[account(mut)]
    pub original_mint: AccountInfo<'info>,
    #[account(mut)]
    pub original_mint_metadata: AccountInfo<'info>,

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
    pub kind: u8,
    pub stake_mint: Option<Pubkey>,
    pub cooldown_start_seconds: Option<i64>,
    pub last_updated_at: Option<i64>,
    pub grouped: Option<bool>,
}