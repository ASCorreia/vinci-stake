use anchor_spl::token::Token;

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
    #[account(mut, seeds = [b"VinciSwap"], bump = vinci_quiz.bump)]
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
    pub fn mega_upgrade(&mut self) -> Result<()> {
        require!(self.vinci_quiz.tournament.iter().find(|&entry| entry.user == self.user.key()).is_some() == true, CustomError::PlayerNotFound);

        let player = self.vinci_quiz.tournament.iter_mut().find(|entry| entry.user == self.user.key()).unwrap();

        require!(player.level == 30, CustomError::InsufficientLevel);

        Ok(())
    }
}