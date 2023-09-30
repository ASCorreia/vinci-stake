use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;

use std::str::FromStr;

use mpl_token_metadata::utils::assert_derivation;
use mpl_token_metadata::state::Metadata;
use mpl_token_metadata::instruction::{freeze_delegated_account, thaw_delegated_account};
use mpl_token_metadata::{self};

use anchor_spl::token::{self};

use vinci_rewards::program::VinciRewards;

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
        ctx.accounts.intialize()?;

        Ok(())
    }

    pub fn initialize_stake_entry(ctx: Context<InitializeStakeEntry>) -> Result<()> {
        ctx.accounts.initialize()?;
        
        ctx.accounts.stake_entry.bump = *ctx.bumps.get("stake_entry").unwrap();

        Ok(())
    }

    pub fn stake(ctx: Context<StakeCtx>) -> Result<()> {
        ctx.accounts.stake_custodial()?;     

        Ok(())
    }

    pub fn stake_non_custodial(ctx: Context<StakeCtx>) -> Result<()> {
        ctx.accounts.stake_non_custodial()?;

        Ok(())
    }

    pub fn claim_stake(ctx: Context<UnstakeCtx>) -> Result<()> {
        ctx.accounts.claim_custodial()?;

        Ok(())
    }

    pub fn claim_non_custodial(ctx: Context<UnstakeCtx>) -> Result<()> {
        ctx.accounts.claim_non_custodial()?;

        Ok(())
    }

    pub fn update_stake(ctx: Context<UpdateStakeCtx>) -> Result<()> { //Does this needs to use remaining accounts???
        //let authority = Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap();
        //require!(signer.key() == authority, CustomError::UnauthorizedSigner);

        //Iterate through all the remaining accounts array
        /*for account in ctx.remaining_accounts.iter() {
            let mut stake_entry_data = account.try_borrow_mut_data()?;

            //Deserialize the data into a stake entry
            let mut stake_entry = StakeEntry::try_deserialize(&mut stake_entry_data.as_ref()).expect("Error deserializing stake entry data");

            //Update the stake time 
            for index in 0..stake_entry.original_mint_seconds_struct.len() {
                let total_stake_seconds = stake_entry.original_mint_seconds_struct[index].time + (stake_entry.total_stake_seconds.saturating_add(
                    (u128::try_from(Clock::get().unwrap().unix_timestamp).unwrap())
                        .saturating_sub(u128::try_from(stake_entry.last_staked_at).unwrap()),
                ));

                stake_entry.original_mint_seconds_struct[index].time = total_stake_seconds;
            }

            //Set the last staked time
            stake_entry.last_staked_at = Clock::get().unwrap().unix_timestamp;

            //Serialize the data back
            stake_entry.try_serialize(&mut stake_entry_data.as_mut())?;
        }*/

        ctx.accounts.update_stake()?;

        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        ctx.accounts.claim_rewards()?;

        Ok(())
    }

    pub fn close_stake_entry(ctx: Context<CloseEntry>) -> Result<()> {
        ctx.accounts.close_stake_entry()?;

        Ok(())
    }

    pub fn close_stake_pool(ctx: Context<ClosePool>) -> Result<()> {
        ctx.accounts.close_stake_pool()?;

        Ok(())
    }
}

// ----- Next Steps ----- //
/*
    1 - Create Stake entry in the pool according to NFT creators (use Metaplex Metadata account to retrieve the creators and make sure they are verified and match the expected account) - Done
        This will need to receive the Token address and the metadata account address (as the program needs to know every account to read / write beforehand) - Done
    2 - If it matches, transfer the NFT to our stake pool (To see the best way to store the user as previous owner (ATA, pubkey??)) - The stake entry shall be validated through creators, 
        and then be used (in another context (maybe stake ctx) to store the initial time, do additional validation and transfer the token). - Done
        Note: Both the original mint account and the final destination shall be know (as the program needs to know every account to read / write beforehand)In progress (Refer to 1. and 2.)
    3 - See how it should update the stack details and the periodic time for that - In progress (User login? Once per day?) - Consider using epochs (client will ask for update once per day)
        Note: Currently there is a Context and method for updating stake entrys already tested!

    Note: Find a way for a user to be able to stake more than 1 NFT in the same pool - Currently done with multiple NFTs per stake entry. 
        Perhaps dig deeper into PDAs and seeds to find another solution (like a user having multiple PDAs? Is it worth it?)?
    
    4. Custodial and non custodial staking (Shall two different operations be used, or just one generic one with a bool argument?) - Currently done with two different functions

    Vinci stake will interact (cpi) with vinci rewards - Done
    Vinci rewards needs to interact (cpi) with vinci accounts - Done
    !!! 3 - Clear no used variables from stack struct - TBD
    !!! 2 - Do a full cycle test (Stake, update, get reward, claim stake) - TBD (Tests to be optimized (readibility))
    !!! 1 - Review main structures for the staking platforms and check wheter the logic can be simplified

    Start considering the possibility of creating a Vinci Dex with SPL tokens and SOL(in progress)
    Add the swap program to the project and deploy it to devnet - Done
    Review tournaments and layouts

 */

