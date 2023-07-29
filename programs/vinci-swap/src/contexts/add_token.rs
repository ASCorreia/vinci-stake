use anchor_lang::system_program;
use anchor_spl::token::Mint;

use crate::*;
//use crate::contexts::error::*;

#[derive(Accounts)]
pub struct AddToken<'info> {
    #[account(mut, seeds = [b"VinciSwap"], bump = vinci_swap.bump)]
    pub vinci_swap: Box<Account<'info, VinciSwap>>,
    pub mint: Box<Account<'info, Mint>>, //SystemAccount to be changed to Account<'info, Mint> SystemAccount<'info>
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddToken<'info> {
    pub fn check_token(&mut self) -> Result<()> {
        let account = &self.vinci_swap;

        match account.assets.iter().find(|mint| mint == &&self.mint.key()) {
            None => {
                self.account_realoc(32, &self.payer, &self.system_program)?;
                self.vinci_swap.assets.push(self.mint.key());
                msg!("Token added to Liquidity Pool");
                Ok(())
            }
            Some(_) => Ok(()),
        }
    }

    pub fn account_realoc(&self, space_to_add: usize, payer: &Signer<'info>, system_program: &Program<'info, System>) -> Result<()> {
        msg!("Reallocating account size to add Token to Liquidity Pool");
        let account_info = self.vinci_swap.to_account_info();
        let new_account_size = account_info.data_len() + space_to_add;

        // Determine additional rent required
        let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
        let additional_rent_to_fund = lamports_required - account_info.lamports();

        // Perform transfer of additional rent
        let cpi_program = system_program.to_account_info();
        let cpi_accounts = system_program::Transfer{
            from: payer.to_account_info(), 
            to: account_info.clone(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        system_program::transfer(cpi_context,additional_rent_to_fund)?;

        // Reallocate the account
        account_info.realloc(new_account_size, false)?;
        msg!("Account Size Updated");

        Ok(())
    } 
}

//consider changing operations to a different file