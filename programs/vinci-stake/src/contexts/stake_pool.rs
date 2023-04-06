use crate::*;

#[derive(Accounts)]
pub struct InitializeStakePool<'info> {
    #[account(init, seeds = [b"VinciWorldStakePool_19", user.key().as_ref()], bump, payer = user, space = 3500)]
    pub stake_pool: Account <'info, StakePool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakePool {
    pub identifier: u64,
    pub authority: Pubkey,
    pub requires_creators: Vec<Pubkey>,
    pub requires_collections: Vec<Pubkey>,
    pub requires_authorization: bool,
    pub overlay_text: String,
    pub image_uri: String,
    pub reset_on_stake: bool,
    pub total_staked: u32,
    pub cooldown_seconds: Option<u32>,
    pub min_stake_seconds: Option<u32>,
    pub end_date: Option<i64>,
    pub double_or_reset_enabled: Option<bool>,
    pub max_stake_amount: Option<u32>,
}