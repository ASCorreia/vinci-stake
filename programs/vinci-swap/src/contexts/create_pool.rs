use anchor_spl::token::{TokenAccount, Token, Transfer};

use crate::*;
//use crate::contexts::error::*;

#[account]
pub struct VinciSwap {
    pub assets: Vec<Pubkey>,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"VinciSwap"], bump, payer = user, space = 8 + (4 + 32) + 1)] //Discriminator + empty Vec<PubKey> + bump(u8)
    pub vinci_swap: Box<Account<'info, VinciSwap>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub trait LiquidityPoolAccount<'info> {
    fn transfer_to_pool(&self, from: &Account<'info, TokenAccount>, to: &Account<'info, TokenAccount>, amount: u64, authority: &Signer<'info>, token_program: &Program<'info, Token>) -> Result<()>;

    fn transfer_from_pool(&self, from: &Account<'info, TokenAccount>, to: &Account<'info, TokenAccount>, amount: u64, token_program: &Program<'info, Token>) -> Result<()>;
}

impl<'info> LiquidityPoolAccount<'info> for Account<'info, VinciSwap> {
    fn transfer_to_pool(&self, from: &Account<'info, TokenAccount>, to: &Account<'info, TokenAccount>, amount: u64, authority: &Signer<'info>, token_program: &Program<'info, Token>) -> Result<()> {
        let cpi_program = token_program.to_account_info();
        let cpi_accounts = Transfer{
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: authority.to_account_info(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::transfer(cpi_context, amount)?;
        Ok(())
    }

    fn transfer_from_pool(&self, from: &Account<'info, TokenAccount>, to: &Account<'info, TokenAccount>, amount: u64, token_program: &Program<'info, Token>) -> Result<()> {
        let seeds = &[
            "VinciSwap".as_bytes(),
            &[self.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = token_program.to_account_info();
        let cpi_accounts = Transfer{
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: self.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        anchor_spl::token::transfer(cpi_context, amount)?;
        Ok(())
    }

}