use vinci_quiz::{program::VinciQuiz, QuizSeason};

use crate::*;

#[derive(Accounts)]
pub struct SeasonRewards<'info> {
    //#[account(seeds = [b"VinciQuiz"], bump = vinci_quiz.bump)]
    pub vinci_quiz: Account<'info, QuizSeason>,
    ///CHECK: This is not dangerous
    #[account(mut)]
    pub owner: Signer<'info>,
    pub quiz_program: Program<'info, VinciQuiz>,
}

#[derive(Accounts)]
pub struct Cenas<'info> {
    #[account(seeds = [b"VinciQuiz"], bump = vinci_quiz.bump)]
    pub vinci_quiz: Account<'info, QuizSeason>,
    pub awarded_1: SystemAccount<'info>,
    pub awarded_2: SystemAccount<'info>,
}