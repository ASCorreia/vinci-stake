use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, Transfer, transfer}};

use crate::*;

#[derive(Accounts)]
pub struct SeasonRewards<'info> {
    #[account(mut, seeds = [b"VinciWorldQuiz"], bump = vinci_quiz.bump)]
    pub vinci_quiz: Box<Account<'info, QuizSeason>>,
    ///CHECK: This is not dangerous as it is only a mint account
    pub mint: UncheckedAccount<'info>,
    #[account(init_if_needed, payer = authority, associated_token::mint = mint, associated_token::authority = vinci_quiz)]
    pub from_ata: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = authority, associated_token::mint = mint, associated_token::authority = user1)]
    pub to_ata_1: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = authority, associated_token::mint = mint, associated_token::authority = user2)]
    pub to_ata_2: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer = authority, associated_token::mint = mint, associated_token::authority = user3)]
    pub to_ata_3: Box<Account<'info, TokenAccount>>,
    pub user1: SystemAccount<'info>,
    pub user2: SystemAccount<'info>,
    pub user3: SystemAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> SeasonRewards<'info> {
    pub fn distribute_rewards(&mut self) -> Result<()> {
        let seeds = &[
            "VinciWorldQuiz".as_bytes(),
            &[self.vinci_quiz.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let mut cpi_program = self.token_program.to_account_info();
        let mut cpi_accounts = Transfer {
            from: self.from_ata.to_account_info(),
            to: self.to_ata_1.to_account_info(),
            authority: self.vinci_quiz.to_account_info(),
        };
        let mut cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, 2_000_000_00)?;

        cpi_program = self.token_program.to_account_info();
        cpi_accounts = Transfer {
            from: self.from_ata.to_account_info(),
            to: self.to_ata_2.to_account_info(),
            authority: self.vinci_quiz.to_account_info(),
        };
        cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, 1_000_000_00)?;

        cpi_program = self.token_program.to_account_info();
        cpi_accounts = Transfer {
            from: self.from_ata.to_account_info(),
            to: self.to_ata_3.to_account_info(),
            authority: self.vinci_quiz.to_account_info(),
        };
        cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, 1_000_000_00)?;

        Ok(())
    }
}