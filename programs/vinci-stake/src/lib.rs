use anchor_lang::prelude::*;
use std::str::FromStr;

use mpl_token_metadata::utils::assert_derivation;
use mpl_token_metadata::state::Metadata;
use mpl_token_metadata::{self};

use anchor_spl::token::{self};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
//HcacNu7JNEtksDekoeHGxdCNGasLtcktayEJbssz2W92

pub mod contexts;
pub mod error;

pub use contexts::*;
pub use error::*;

#[program]
pub mod vinci_stake {
    use anchor_lang::solana_program::stake::state::Stake;

    use super::*;

    pub fn initialize_stake_pool(ctx: Context<InitializeStakePool>) -> Result<()> {

        let stake_pool = &mut ctx.accounts.stake_pool;

        stake_pool.double_or_reset_enabled = None;
        stake_pool.cooldown_seconds = None;
        stake_pool.identifier = 0xBEBACAFE;
        stake_pool.requires_authorization = false;
        stake_pool.requires_creators.push(Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap());

        Ok(())
    }

    pub fn initialize_stake_entry(ctx: Context<InitializeStakeEntry>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool_account;
        let stake_entry = &mut ctx.accounts.stake_entry;

        stake_entry.original_mint = ctx.accounts.original_mint.key();
        stake_entry.pool = stake_pool.key();
        stake_entry.amount = 0; //Probably not needed

        // assert metadata account derivation
        assert_derivation(
            &mpl_token_metadata::id(),
            &ctx.accounts.original_mint_metadata.to_account_info(),
            &[
                mpl_token_metadata::state::PREFIX.as_bytes(),
                mpl_token_metadata::id().as_ref(),
                ctx.accounts.original_mint.key().as_ref(),
            ],
        )?;

        require!(ctx.accounts.original_mint_metadata.data_is_empty() == false, CustomError::MetadataAccountEmpty);

        /* Borrow and deserialize the metada account from the original mint metadata */
        let mint_metadata_data = ctx.accounts.original_mint_metadata.try_borrow_mut_data().expect("Error borrowing data");
        require!(ctx.accounts.original_mint_metadata.to_account_info().owner.key() == mpl_token_metadata::id(), CustomError::InvalidMintOwner); //Checks that the owner is the Metadadata program
        let original_mint_metadata = Metadata::deserialize(&mut mint_metadata_data.as_ref()).expect("Error deserializng metadata");
        require!(original_mint_metadata.mint == ctx.accounts.original_mint.key(), CustomError::InvalidMint); //Checks that both the original mint and the one stored i nthe account are the same

        //Get the creators from the metadata and see if the it contains the ones required by the stake pool
        //let creators = original_mint_metadata.data.creators.unwrap();
        //let find_creators = creators.iter().find(|creator| stake_pool.requires_creators.contains(&creator.address) && creator.verified);

        //Checks that the creators have been found
        //require!(find_creators.is_some() == true, CustomError::MissingCreators);   

        Ok(())
    }

    pub fn stake(ctx: Context<StakeCtx>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool;
        let stake_entry = &mut ctx.accounts.stake_entry;

        //TBD Do checks to the stake accounts and add more custom errors

        let from_token_account = &mut ctx.accounts.from_mint_token_account;
        let to_token_account = &mut ctx.accounts.to_mint_token_account;

        // Transfer NFT
        let cpi_accounts = token::Transfer {
            from: from_token_account.to_account_info(),
            to: to_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_context, 1)?;

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

    The stake entry shall be validated through creators, and then be used (in another context (maybe stake ctx) to store the initial time, do additional validation and transfer the token).
    Note: Both the original mint account and the final destination shall be know (as the program needs to know every account to read / write beforehand)
 */

