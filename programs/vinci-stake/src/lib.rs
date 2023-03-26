use mpl_token_metadata::utils::assert_derivation;

use anchor_lang::prelude::*;
use std::str::FromStr;
use mpl_token_metadata::state::Metadata;
use mpl_token_metadata::{self};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
//HcacNu7JNEtksDekoeHGxdCNGasLtcktayEJbssz2W92

pub mod contexts;

pub use contexts::*;

#[program]
pub mod vinci_stake {
    use super::*;

    pub fn initialize_stake_pool(ctx: Context<InitializeStakePool>) -> Result<()> {

        let _stake_pool = &mut ctx.accounts.stake_pool;

        Ok(())
    }

    pub fn initialize_stake_entry(ctx: Context<InitializeStakeEntry>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool;
        let stake_entry = &mut ctx.accounts.stake_entry;

        stake_entry.original_mint = ctx.accounts.original_mint.key();
        stake_entry.pool = stake_pool.key();
        stake_entry.amount = 0; //Probably not needed

        Ok(())
    }
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

// ----- Next Steps ---- //
/*
    1 - Create Stake entry in the pool according to NFT creators (use Metaplex Metadata account to retrieve the creators and make sure they are verified and match the expected account)
        This will need to receive the Token address and the metadata account address (as the program needs to know every account to read / write beforehand)
    2 - If it matches, transfer the NFT to our stake pool (To see the best way to store the user as previous owner (ATA, pubkey??))
    3 - See how it should update the stack details and the periodic time for that
 */

