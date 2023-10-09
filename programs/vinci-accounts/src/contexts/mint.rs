use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token::Mint;

use crate::*;

#[derive(Accounts)]
pub struct MintNFT<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    #[account(mut, seeds = [b"authority"], bump)]
    pub mint_authority: SystemAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintNFT<'info> {
    /*
        for details regarding the metadata account and master edition account, refer to metaplex docs at
        https://docs.metaplex.com/programs/token-metadata/accounts
    */
    pub fn mint_nft(&mut self, uri: String, title: String) -> Result<()> {
        /* Create a CPI Call to mint one token */
        msg!("Initializing Mint NFT");
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = self.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let (_pda_address, pda_bump) = Pubkey::find_program_address(&[b"authority"], &id());
        let seeds = &[
            "authority".as_bytes(),
            &[pda_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");

        /* Create an Account Info vector to hold the necessary accounts to create a Token Metadata Account */
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
        /* Create the NFT creators vector */
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: self.mint_authority.key(),
                verified: true,
                share: 100,
            },
        ];
        msg!("Creator Assigned");

        /* Create a CPI call to create a Metadata Account associated with our minted token */
        let symbol = std::string::ToString::to_string("NNV");
        invoke_signed(
            &create_metadata_accounts_v3(
                self.token_metadata_program.key(), //program_id
                self.metadata.key(), //metadata_account
                self.mint.key(), //mint
                self.mint_authority.key(), //mint_authority
                self.payer.key(), //payer
                self.mint_authority.key(), //update_authority
                title, //name
                symbol, //symbol
                uri, //uri
                Some(creator), //creators
                500, //seller_fee_basis_points
                true, //update_authority_is_signer
                true, //is_mutable
                None, //collection
                None, //uses
                None, //collection_details
            ),
            account_info.as_slice(),
            signer_seeds,
        )?;
        msg!("Metadata Account Created !!!");
        
        /* Create an Account Info vector to hold the necessary accounts to create a Master Edition Account */
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
        /* Create a CPI call to create a Master Edition associated with our minted token and our metadata account */
        invoke_signed(
            &create_master_edition_v3(
                self.token_metadata_program.key(), //program_id
                self.master_edition.key(), //edition
                self.mint.key(), //mint
                self.mint_authority.key(), //update_authority
                self.mint_authority.key(), //mint_authority
                self.metadata.key(), //metadata (metadata_account)
                self.payer.key(), //payer
                Some(0), //max_supply
            ),
            master_edition_infos.as_slice(),
            signer_seeds,
        )?;
        msg!("Master Edition Nft Minted !!!");
        Ok(())
    }
}