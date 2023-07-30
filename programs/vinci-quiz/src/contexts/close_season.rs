use crate::*;

#[derive(Accounts)]
pub struct CloseSeason<'info> {
    #[account(mut, close = destination)]
    pub vinci_quiz: Account<'info, QuizSeason>,
    #[account(mut)]
    pub destination: SystemAccount<'info>,
    pub authority: Signer<'info>,
}

impl<'info> CloseSeason<'info> {
    pub fn close_season(&self) -> Result<()> {
        msg!("Vinci Quiz Season successfully closed");

        Ok(())
    }
}