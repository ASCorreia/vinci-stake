use anchor_lang::solana_program::program::invoke_signed;
use mpl_token_metadata::{instruction::update_metadata_accounts_v2, state::DataV2};

use crate::*;

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    ///CHECK:: This is not dangerous as we do not read or write from this account
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    pub authority: SystemAccount<'info>,
    #[account(mut)]
    //TBD add signer responsible for this change
    pub payer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: AccountInfo<'info>,

}

impl<'info> UpdateMetadata<'info> {
    pub fn update_metadata(&mut self, uri: String) -> Result<()> {
        /* Derive our auhtority PDA */
        let (_pda_address, pda_bump) = Pubkey::find_program_address(&[b"authority"], &id());
        let seeds = &[
            "authority".as_bytes(),
            &[pda_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        /* Create an Account Info vector to hold the necessary accounts to perform a CPI to change our NFT Metadata */
        let account_info = vec![
            self.metadata.to_account_info(),
            self.authority.to_account_info(),
            self.token_metadata_program.to_account_info(),
        ];

        /* Create a Creator vector */
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: self.authority.key(),
                verified: true,
                share: 100,
            },
        ];
        msg!("Creator Assigned");

        /* Create the DataV2 struct to associate with the token */
        let data = DataV2 {
            name: "NN Vinci Model".to_owned(),
            symbol: "NNV".to_owned(),
            uri,
            seller_fee_basis_points: 500,
            creators: Some(creator),
            collection: None,
            uses: None,
        };

        /* Perform a CPI to change the NFT Metadata */
        invoke_signed(
            &update_metadata_accounts_v2(
                self.token_metadata_program.key(), 
                self.metadata.key(), 
                self.authority.key(), 
                None, 
                Some(data), 
                None,
                None
            ), 
            account_info.as_slice(),
            signer_seeds,
        )?;
        
        Ok(())
    }
}