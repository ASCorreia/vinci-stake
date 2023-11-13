use crate::*;

use anchor_spl::token::{Token, TokenAccount, Mint, Burn};

#[derive(Accounts)]
pub struct MintToken<'info> {
    pub token_program: Program<'info, Token>,
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: Account<'info, Mint>, //Token Account (Represents the token)
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, //Destination of the mint. The token that we want to send to tokens to!
    #[account(mut)]
    pub payer: Signer<'info> //Authority to mint the token
}

impl<'info> MintToken <'info> {
    pub fn mint_token(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.payer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts); //add a CPI context with signer (CpiContext::new_with_signer) for the user to sign (signer_seeds? Try PubKey)

        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnToken<'info> {
    pub token_program: Program<'info, Token>,
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: Account<'info, Mint>, //Token Account (Represents the token)
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, //Destination of the mint. The token that we want to send tokens to!
    #[account(mut)]
    pub payer: Signer<'info> //Authority to mint the token
}

impl<'info> BurnToken<'info> {
    pub fn burn_token(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: self.mint.to_account_info(),
            from: self.token_account.to_account_info(),
            authority: self.payer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, amount)?;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    pub token_program: Program<'info, Token>,
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: Account<'info, Mint>, //Token Account (Represents the token)
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, //Destination of the mint. The token that we want to send to tokens to!
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //Authority to mint the token (Shall be the Signer as well)
}

impl<'info> ClaimTokens<'info> {
    pub fn claim_tokens(&mut self) -> Result<()> {
        let account_to_claim = &mut self.base_account;
        
        //Security check to be used when the frontend / backend is updated 
        //require!(ctx.accounts.payer.is_signer == true && ctx.accounts.payer.to_account_info().key() == account_to_claim.owner, CustomError::WrongSigner);
    
        require!(account_to_claim.total_amount != 0, CustomError::InsufficientBalanceSpl);
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.payer.to_account_info(),
        };
    
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
        token::mint_to(cpi_ctx, account_to_claim.total_amount)?;
        account_to_claim.total_amount = 0;

        Ok(())
    }
}