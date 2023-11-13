use crate::*;

pub trait QuizFuncs<'info> {
    fn order_entries(&mut self) -> Result<()>;
}

impl<'info> QuizFuncs<'info> for Account<'info, QuizSeason> {
    fn order_entries(&mut self) -> Result<()> {
            self.tournament.sort_by_key(|entry|entry.score.clone());
            self.tournament.reverse();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"VinciWorldQuiz"], bump, payer = user, space = 8 + (4 + 46) + 4 + 1)]
    pub vinci_quiz: Account<'info, QuizSeason>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}