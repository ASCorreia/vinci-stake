use crate::*;

#[derive(Accounts)]
pub struct CloseSeason<'info> {
    #[account(mut, close = destination)]
    vinci_quiz: Account<'info, QuizSeason>,
    #[account(mut)]
    destination: SystemAccount<'info>,
}

impl<'info> CloseSeason<'info> {
    pub fn close_season(&self) -> Result<()> {
        msg!("Vinci Quiz Season successfully closed");

        Ok(())
    }
}