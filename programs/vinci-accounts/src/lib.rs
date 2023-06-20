use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{Token, InitializeMint, MintTo, Burn, Transfer};
use std::str::FromStr;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};

//declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("38N2x62nEqdgRf67kaemiBNFijKMdnqb3XyCa4asw2fQ");

pub mod contexts;
pub mod error;

pub use contexts::*;
pub use error::*;

#[program]
pub mod vinci_accounts {

    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        let pubkey = Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap();
        let base_account = &mut ctx.accounts.base_account;
        let result = base_account.key();
        msg!(&result.to_string());
        base_account.total_amount = 0;
        base_account.owner = pubkey;

        Ok(())
    }

    /*  
        TBD Create funtion to transfer tokens from accounts to vault (our account) to be used for tournament registration.
        ATAs should be created directly from the client during the baseAccount creation. In that case, the program would only get the address
        
        TBD Create function to transfer tokens from vault to accounts (from an ATA owned by our wallet)
        Signer should be our wallet, signed from the client
    */
    
    pub fn mint_token(ctx: Context<MintToken>, ammount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts); //add a CPI context with signer (CpiContext::new_with_signer) for the user to sign (signer_seeds? Try PubKey)

        token::mint_to(cpi_ctx, ammount)?;
        Ok(())
    }

    /* 
        TBD At the moment the function is receiving the computed PDA. To be discussed if it is the best approach
    */
    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        let account_to_claim = &mut ctx.accounts.base_account;
        
        //Security check to be used when the frontend / backend is updated 
        //require!(ctx.accounts.payer.is_signer == true && ctx.accounts.payer.to_account_info().key() == account_to_claim.owner, CustomError::WrongSigner);
    
        require!(account_to_claim.total_amount != 0, CustomError::InsufficientBalanceSpl);
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
    
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
        token::mint_to(cpi_ctx, account_to_claim.total_amount)?;
        account_to_claim.total_amount = 0;

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

            if ctx.accounts.user.is_signer == true  && ctx.accounts.user.to_account_info().key() == account_to_write.owner {
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

    pub fn start_tournament(ctx: Context<StartTournament>, prize_pool: u32) -> Result<()> {
        let pubkey = Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap(); //to be updated with appropriate wallet PubKey

        let tournament = &mut ctx.accounts.tournament;
        tournament.tournament_list = Vec::new();
        tournament.owner = pubkey;
        tournament.prize_pool = prize_pool;

        Ok(())
    }

    pub fn add_tournament_participant(ctx: Context<AddPartcipant>) -> Result<()> {
        let base_account = &mut ctx.accounts.new_participant;
        let tournament_list = &mut ctx.accounts.tournament_list;

        require!(ctx.accounts.user.is_signer == true && ctx.accounts.user.key() == tournament_list.owner, CustomError::WrongSigner);
        if !tournament_list.tournament_list.iter().any(|t| t.user == base_account.key()) {
            tournament_list.tournament_list.push(TournamentStruct { user: base_account.key(), score: 0 });
        }
        Ok(())
    }

    /*
        TBD Create a new tournament payout function based on the new tournament account.
    */

    pub fn add_ammount(ctx: Context<AddAmount>, ammount: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        
        //Security check to be used when the frontend / backend is updated 
        //require!(ctx.accounts.owner.is_signer == true && *ctx.accounts.owner.key == base_account.owner, CustomError::WrongSigner);

        base_account.total_amount += ammount;
        Ok(())
    }

    pub fn burn_token(ctx: Context<BurnToken>, ammount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, ammount)?;
        Ok(())
    }

    pub fn remove_ammount(ctx: Context<RemoveAmmount>, ammount: u64) -> Result<()> {
       
        let base_account = &mut ctx.accounts.base_account;
        require!(ammount <= base_account.total_amount, CustomError::InsufficientBalanceSpl);
        require!(ctx.accounts.owner.is_signer == true && *ctx.accounts.owner.key == base_account.owner, CustomError::WrongSigner);
        
        base_account.total_amount -= ammount;
        
        Ok(())
    }

    /*
        for details regarding the metadata account and master edition account, refer to metaplex docs at
        https://docs.metaplex.com/programs/token-metadata/accounts
     */
    pub fn mint_nft(ctx: Context<MintNFT>, creator_key: Pubkey, uri: String, title: String) -> Result<()> {
        msg!("Initializing Mint NFT");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("VINCI");
        invoke(
            &create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(), //program_id
                ctx.accounts.metadata.key(), //metadata_account
                ctx.accounts.mint.key(), //mint
                ctx.accounts.mint_authority.key(), //mint_authority
                ctx.accounts.payer.key(), //payer
                ctx.accounts.payer.key(), //update_authority
                title, //name
                symbol, //symbol
                uri, //uri
                Some(creator), //creators
                500, //seller_fee_basis_points
                true, //update_authority_is_signer
                false, //is_mutable
                None, //collection
                None, //uses
                None, //collection_details
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(), //program_id
                ctx.accounts.master_edition.key(), //edition
                ctx.accounts.mint.key(), //mint
                ctx.accounts.payer.key(), //update_authority
                ctx.accounts.mint_authority.key(), //mint_authority
                ctx.accounts.metadata.key(), //metadata (metadata_account)
                ctx.accounts.payer.key(), //payer
                Some(0), //max_supply
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");
        Ok(())
    }
}
