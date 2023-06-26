use crate::*;
use anchor_spl::{token::{Mint, Token, TokenAccount, Transfer}, associated_token::AssociatedToken};
//use crate::contexts::error::*;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(seeds = [b"VinciSwap"], bump = vinci_swap.bump)]
    pub vinci_swap: Box<Account<'info, VinciSwap>>,
    #[account(mut)]
    pub owner_ata: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = user, associated_token::mint = token_mint, associated_token::authority = vinci_swap)]
    pub vault_ata: Box<Account<'info, TokenAccount>>,
    pub token_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(mut)]
    pub user: Signer<'info>,
}

impl<'info> AddLiquidity<'info> {
    pub fn transfer_to_pool(&self, amount: u64) -> Result<()> {
        let mint_found = self.vinci_swap.assets.iter().find(|mint| mint == &&self.token_mint.key()).is_some();
        require!(mint_found == true, CustomError::NoPoolFound);

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.owner_ata.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        msg!("{:?} Tokens successfully sent to the vault", amount);

        Ok(())
    }  
}