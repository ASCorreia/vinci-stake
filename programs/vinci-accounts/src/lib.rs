use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, MintTo};
use std::str::FromStr;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};

declare_id!("38N2x62nEqdgRf67kaemiBNFijKMdnqb3XyCa4asw2fQ");

pub mod contexts;
pub mod error;

pub use contexts::*;
pub use error::*;

#[program]
pub mod vinci_accounts {

    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        ctx.accounts.start_stuff_off()?;

        ctx.accounts.base_account.bump = *ctx.bumps.get("base_account").unwrap();

        Ok(())
    }
    
    pub fn mint_token(ctx: Context<MintToken>, ammount: u64) -> Result<()> {
        ctx.accounts.mint_token(ammount)?;

        Ok(())
    }

    pub fn burn_token(ctx: Context<BurnToken>, ammount: u64) -> Result<()> {
        ctx.accounts.burn_token(ammount)?;

        Ok(())
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        ctx.accounts.claim_tokens()?;

        Ok(())
    }

    pub fn add_ammount(ctx: Context<Operations>, ammount: u64) -> Result<()> {
        ctx.accounts.add_amount(ammount)?;
        
        Ok(())
    }

    pub fn set_score(ctx: Context<Operations>, score: u64) -> Result<()> {
        ctx.accounts.set_score(score)?;
        
        Ok(())
    }

    pub fn remove_ammount(ctx: Context<Operations>, ammount: u64) -> Result<()> {
        ctx.accounts.remove_amount(ammount)?;
        
        Ok(())
    }

    pub fn start_tournament(ctx: Context<StartTournament>, prize_pool: u32) -> Result<()> {
        ctx.accounts.start_tournament(prize_pool)?;

        Ok(())
    }

    pub fn add_tournament_participant(ctx: Context<AddPartcipant>) -> Result<()> {
        ctx.accounts.add_tournament_participant()?;

        Ok(())
    }

    ///pay_tournament function will deserialize the provided remaining accounts in order to add the rewarded ammount to the appropriate account
    pub fn pay_tournament(ctx:Context<PayTournament>, ammount: u64) -> Result<()>
    {
        /*
            Tournament payout function shall be updated in the future with a proper payout structure (still according to the total amount of participants).
            Has there is not yet a defined final structure, the following win account are just used for reference and are not the final values.  
        */   
        let win_accounts: usize;

        let total_accounts: usize = ctx.remaining_accounts.len();      
        if total_accounts != 1 as usize {
            panic!("Total accounts is {}", total_accounts); //Only for debugging / testing purposes. No panic! intended to be used
        }
        if total_accounts  >= 1 && total_accounts <= 10 { //1 to be replaced by appropriate number
            win_accounts = 1;
        }
        else if total_accounts > 10 && total_accounts <= 24 {
            win_accounts = 6;
        }
        else {
            win_accounts = 10;
        }

        let mut awarded_accounts = 0;
        for account in ctx.remaining_accounts.iter() {

            let _account_key = account.key();
            let mut data = account.try_borrow_mut_data()?;
            //let data_to_write = data.as_ref();

            //Deserialize the data from the account and save it in an Account variable
            let mut account_to_write = BaseAccount::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");

            if ctx.accounts.user.is_signer == true  && ctx.accounts.user.to_account_info().key() == account_to_write.authority {
                if awarded_accounts != win_accounts {
                    account_to_write.total_amount += ammount;
                    awarded_accounts += 1;
                }
            }
           
            //Serialize the data back
            account_to_write.try_serialize(&mut data.as_mut())?;
        }

        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNFT>, uri: String, title: String) -> Result<()> {
        ctx.accounts.mint_nft(uri, title)?;

        Ok(())
    }

    pub fn season_rewards(ctx: Context<SeasonRewards>) -> Result<()> {       
        msg!("Vinci Quiz Key: {:?}", ctx.accounts.vinci_quiz.key().to_string());

        let seeds = &[
            "VinciQuiz".as_bytes(),
        ];
        let (pda, bump) = anchor_lang::prelude::Pubkey::find_program_address(seeds, &ctx.accounts.quiz_program.key());
        msg!("Derived PDA is {:?} and bump is {:?}", pda.to_string(), bump);

        require!((ctx.accounts.vinci_quiz.key() == pda), CustomError::WrongPDA);
        require!(ctx.accounts.vinci_quiz.bump == bump, CustomError::WrongBump);

        for account in ctx.remaining_accounts.iter() {

            let account_key = account.key();
            let mut data = account.try_borrow_mut_data()?;

            //Deserialize the data from the account and save it in an Account variable
            let mut account_to_write = BaseAccount::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");

            account_to_write.total_amount += 500;
           
            //Serialize the data back
            account_to_write.try_serialize(&mut data.as_mut())?;

            msg!("Account {:?} has been awarded 500 Vinci Points", account_key.to_string());
        }
        
        Ok(())
    }

    pub fn update_metadata(ctx: Context<UpdateMetadata>, uri: String) -> Result<()> {
        ctx.accounts.update_metadata(uri)?;

        Ok(())
    }

    pub fn close_account(ctx: Context<Close>) -> Result<()> {
        let _ctx = ctx;
        
        Ok(())
    }
}
