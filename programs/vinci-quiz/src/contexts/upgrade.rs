use anchor_spl::token::{Token, MintTo, self};
use solana_program::program::invoke;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};

use crate::*;

#[derive(Accounts)]
pub struct Upgrade<'info> {
    #[account(mut, seeds = [b"VinciQuiz"], bump = vinci_quiz.bump)]
    pub vinci_quiz: Account<'info, QuizSeason>,
    pub user: SystemAccount<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct MegaUpgrade<'info> {
    #[account(mut, seeds = [b"VinciQuiz"], bump = vinci_quiz.bump)]
    pub vinci_quiz: Account<'info, QuizSeason>,
    pub user: SystemAccount<'info>,
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    //#[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}

impl<'info> Upgrade<'info> {
    pub fn upgrade(&mut self) -> Result<()> {
        require!(self.vinci_quiz.tournament.iter().find(|&entry| entry.user == self.user.key()).is_some() == true, CustomError::PlayerNotFound);

        let player = self.vinci_quiz.tournament.iter_mut().find(|entry| entry.user == self.user.key()).unwrap();

        require!(player.score >= 30, CustomError::InsufficientPoints);

        match player.level {
            u8::MIN..=9 => player.level += 1,
            _ => return Ok(()),
        }
        msg!("Player level has been !!!UPGRADED!!! to level {:?}", player.level);

        player.score -= 30;
        
        Ok(())
    }
}

impl<'info> MegaUpgrade<'info> {
    pub fn mega_upgrade(&mut self, creator_key: Pubkey, uri: String, title: String) -> Result<()> {
        require!(self.vinci_quiz.tournament.iter().find(|&entry| entry.user == self.user.key()).is_some() == true, CustomError::PlayerNotFound);

        let player = self.vinci_quiz.tournament.iter_mut().find(|entry| entry.user == self.user.key()).unwrap();

        require!(player.level == 10, CustomError::InsufficientLevel);

        require!(player.nft_minted == false, CustomError::NftAlreadyMinted);

        msg!("Initializing Mint NFT");
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.payer.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = self.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            self.metadata.to_account_info(),
            self.mint.to_account_info(),
            self.mint_authority.to_account_info(),
            self.payer.to_account_info(),
            self.token_metadata_program.to_account_info(),
            self.token_program.to_account_info(),
            self.system_program.to_account_info(),
            self.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: self.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("VINCI");
        invoke(
            &create_metadata_accounts_v3(
                self.token_metadata_program.key(), //program_id
                self.metadata.key(), //metadata_account
                self.mint.key(), //mint
                self.mint_authority.key(), //mint_authority
                self.payer.key(), //payer
                self.payer.key(), //update_authority
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
            self.master_edition.to_account_info(),
            self.mint.to_account_info(),
            self.mint_authority.to_account_info(),
            self.payer.to_account_info(),
            self.metadata.to_account_info(),
            self.token_metadata_program.to_account_info(),
            self.token_program.to_account_info(),
            self.system_program.to_account_info(),
            self.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                self.token_metadata_program.key(), //program_id
                self.master_edition.key(), //edition
                self.mint.key(), //mint
                self.payer.key(), //update_authority
                self.mint_authority.key(), //mint_authority
                self.metadata.key(), //metadata (metadata_account)
                self.payer.key(), //payer
                Some(0), //max_supply
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");

        player.nft_minted = true;

        Ok(())
    }
}