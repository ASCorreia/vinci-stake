use anchor_lang::prelude::*;
use std::str::FromStr;

use mpl_token_metadata::utils::assert_derivation;
use mpl_token_metadata::state::Metadata;
use mpl_token_metadata::{self};

use anchor_spl::token::{self};

declare_id!("EjhezvQjSDBEQXVyJSY1EhmqsQFGEorS7XwwHmxcRNxV");
//HcacNu7JNEtksDekoeHGxdCNGasLtcktayEJbssz2W92
//Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS

pub mod contexts;
pub mod error;

pub use contexts::*;
pub use error::*;

#[program]
pub mod vinci_stake {
    
    use super::*;

    pub fn initialize_stake_pool(ctx: Context<InitializeStakePool>) -> Result<()> {

        let stake_pool = &mut ctx.accounts.stake_pool;

        stake_pool.double_or_reset_enabled = None;
        stake_pool.cooldown_seconds = None;
        stake_pool.identifier = 0xBEBACAFE;
        stake_pool.requires_authorization = false;
        stake_pool.requires_creators.push(Pubkey::from_str("7qZkw6j9o16kqGugWTj4u8Lq9YHcPAX8dgwjjd9EYrhQ").unwrap());
        stake_pool.max_stake_amount = None;
        stake_pool.total_staked = 0;

        Ok(())
    }

    pub fn initialize_stake_entry(ctx: Context<InitializeStakeEntry>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool_account;
        let stake_entry = &mut ctx.accounts.stake_entry;

        stake_entry.original_mint = ctx.accounts.original_mint.key();
        stake_entry.pool = stake_pool.key();
        stake_entry.amount = 0; //Probably not needed
        stake_entry.original_mint_claimed = Vec::new();
        stake_entry.stake_mint_claimed = Vec::new();
        stake_entry.original_mint_seconds_struct = Vec::new();
        stake_entry.original_owner = ctx.accounts.user.key();

        // assert metadata account derivation (asserts from a programID, an account and a path (seeds))
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
        let creators = original_mint_metadata.data.creators.unwrap();
        let find_creators = creators.iter().find(|creator| stake_pool.requires_creators.contains(&creator.address) && !creator.verified); // (!)creator.verified

        //Checks that the creators have been found
        require!(find_creators.is_some() == true, CustomError::MissingCreators);   

        Ok(())
    }

    pub fn stake(ctx: Context<StakeCtx>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool;

        let stake_entry = &mut ctx.accounts.stake_entry;
        let original_mint = stake_entry.original_mint.key();

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

        //token::approve(ctx, amount) - For non custodial staking

        stake_entry.original_owner = from_token_account.key();
        stake_entry.staking_owner = to_token_account.key();

        //Set the last staked time
        stake_entry.last_staked_at = Clock::get().unwrap().unix_timestamp;
        
        //Update the total staked time
        stake_entry.total_stake_seconds = stake_entry.total_stake_seconds.saturating_add(
            (u128::try_from(Clock::get().unwrap().unix_timestamp).unwrap())
                .saturating_sub(u128::try_from(stake_entry.last_staked_at).unwrap()),
        );

        //Flag that the original mint has been claimed by the pool
        stake_entry.original_mint_claimed.push(original_mint);

        /* The following is an approach to store staking time if we decide to have multiple mints per entry */
        let staking_time = StakeTime{time: stake_entry.total_stake_seconds, mint: original_mint};
        stake_entry.original_mint_seconds_struct.push(staking_time);
        /* ------------------------------------------------------------------------------------------------ */

        stake_pool.total_staked += 1;

        

        Ok(())
    }

    pub fn stake_non_custodial(ctx: Context<StakeCtx>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool;

        let stake_entry = &mut ctx.accounts.stake_entry;
        let original_mint = stake_entry.original_mint.key();

        let user_token_accout = &mut ctx.accounts.from_mint_token_account;
        let delegate = &mut ctx.accounts.to_mint_token_account; //to be replaced by the program address
        let authority = &mut ctx.accounts.user;

        let cpi_accounts = token::Approve {
            to: user_token_accout.to_account_info(),
            delegate: delegate.to_account_info(),
            authority: authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token::approve(cpi_context, 1)?;

        /*TBD:
        refer to https://github.com/metaplex-foundation/metaplex-program-library/blob/7c4ceb0100364901f2317f3421aab0aac400b647/token-metadata/program/src/processor.rs#L1552-L1606
        in order to freeze delegated account
        */

        stake_pool.total_staked += 1;
        

        Ok(())
    }

    pub fn claim_stake(ctx: Context<StakeCtx>) -> Result<()> {
        //let authority = Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap();

        let stake_entry = &mut ctx.accounts.stake_entry;

        let from_token_account = &mut ctx.accounts.from_mint_token_account;
        let to_token_account = &mut ctx.accounts.to_mint_token_account;

        let original_mint = &mut ctx.accounts.original_mint;

        let signer = &mut ctx.accounts.user;

        //require!(stake_entry.original_mint_claimed.iter().find(|mint| **mint == original_mint.key()) == Some(&original_mint.key()), CustomError::OriginalMintNotClaimed);
        //require!(stake_entry.stake_mint_claimed.iter().find(|mint| **mint == original_mint.key()) == None, CustomError::MintAlreadyClaimed);
        //require!(signer.key() == authority, CustomError::UnauthorizedSigner);

        //Transfer NFT
        let cpi_accounts = token::Transfer{
            from: from_token_account.to_account_info(),
            to: to_token_account.to_account_info(),
            authority: signer.to_account_info(),
        };       
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_context, 1)?;

        stake_entry.stake_mint_claimed.push(original_mint.key());
        stake_entry.original_mint_claimed.retain(|mint| *mint != ctx.accounts.original_mint.key());
        /* The following is an approach to store staking time if we decide to have multiple mints per entry */
        //stake_entry.original_mint_seconds_struct.retain(|stake_mint_struct| stake_mint_struct.mint != ctx.accounts.original_mint.key());
        /* ------------------------------------------------------------------------------------------------ */
        stake_entry.total_stake_seconds = 0;

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
    1 - Create Stake entry in the pool according to NFT creators (use Metaplex Metadata account to retrieve the creators and make sure they are verified and match the expected account) - Done
        This will need to receive the Token address and the metadata account address (as the program needs to know every account to read / write beforehand) - Done
    2 - If it matches, transfer the NFT to our stake pool (To see the best way to store the user as previous owner (ATA, pubkey??)) - The stake entry shall be validated through creators, 
        and then be used (in another context (maybe stake ctx) to store the initial time, do additional validation and transfer the token).
        Note: Both the original mint account and the final destination shall be know (as the program needs to know every account to read / write beforehand)In progress (Refer to 1. and 2.)
    3 - See how it should update the stack details and the periodic time for that - In progress (User login? Once per day?)
    4 - Create the update stake time function

    Note: Find a way for a user to be able to stake more than 1 NFT in the same pool (how to create different PDAs (stake entry) for the same user in the same pool (try look at the 
        anchor init seeds)

    1. Try to use an array of original mint claimed, to be updated wih the original mint (so an user can have an unique stake entry with different tokens)
        (consider both stake claimed and original mint claimed)
        Note: Find a way for a user to be able to stake more than 1 NFT in the same pool (how to create different PDAs (stake entry) for the same user in the same pool (try look at the anchor init seeds)
    2. Custodial and non custodial staking (Shall two different operations be used, or just one generic one with a bool argument?)
        Refer to token::approve and mpl_token_metadata::freeze_delegated_account

 */

