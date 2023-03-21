use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vinci_stake {
    use super::*;

    pub fn initialize_stake_pool(ctx: Context<InitializeStakePool>) -> Result<()> {

        let _stake_pool = &mut ctx.accounts.stake_pool;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeStakePool<'info> {
    #[account(init, seeds = [b"VinciWorldStakePool", user.key().as_ref()], bump, payer = user, space = 3500)]
    stake_pool: Account <'info, StakePool>,
    #[account(mut)]
    user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GroupStakeEntry {
    pub bump: u8,
    pub group_id: Pubkey,
    pub authority: Pubkey,
    pub stake_entries: Vec<Pubkey>,
    pub changed_at: i64,
    pub group_cooldown_seconds: u32,
    pub group_stake_seconds: u32,
    pub group_cooldown_start_seconds: Option<i64>,
}

#[account]
pub struct StakeEntry {
    pub bump: u8,
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

#[account]
pub struct StakePool {
    pub bump: u8,
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
}

