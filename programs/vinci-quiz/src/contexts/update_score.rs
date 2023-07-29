use crate::*;

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    #[account(mut, seeds = [b"VinciQuiz"], bump = vinci_quiz.bump)]
    pub vinci_quiz: Account<'info, QuizSeason>,
    pub user: SystemAccount<'info>,
}

impl<'info> UpdateScore<'info> {
    pub fn update_score(&mut self, score: u32) -> Result<()> {
        require!(self.vinci_quiz.tournament.iter().find(|player_entry| player_entry.user == self.user.key()).is_some() == true, CustomError::PlayerNotFound);

        let tournament = &mut self.vinci_quiz.tournament;
        let player_entry = tournament.iter_mut().find(|player_entry| player_entry.user == self.user.key()).unwrap();

        match score {
            30..=u32::MAX => player_entry.score += score,
            _ => match player_entry.score {
                20..=u32::MAX => player_entry.score -= score,
                _ => return Ok(()),
            }
        }

        Ok(())
    }

    pub fn xorshift64star(&mut self, seed: u64) -> u64 {
        let mut x = seed;
        x ^= x << 12;
        x ^= x >> 25;
        x ^= x << 27;
        x = (x as u128 * 0x2545F4914F6CDD1D) as u64;

        x
    }
}