use anchor_lang::system_program;

use crate::*;

#[derive(Accounts)]
pub struct AddPlayer<'info> {
    #[account(mut, seeds = [b"VinciWorldQuiz"], bump = vinci_quiz.bump)]
    pub vinci_quiz: Account<'info, QuizSeason>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddPlayer<'info> {
    pub fn add_player(&mut self) -> Result<()> {
        
        match self.vinci_quiz.tournament.iter().find(|entry| entry.user == self.user.key()) {
            None => {
                self.realloc(46, &self.user, &self.system_program)?;
                self.vinci_quiz.tournament.push(UserEntry{ score: 0, user: self.user.key(), level: 1, nft_minted: false });
                self.vinci_quiz.entries += 1;
                msg!("Player Succesfully added to Vinci Quiz Season");
                Ok(())
            }
            Some(_) => {
                msg!("Player already registered to Vinci Quiz Season");
                Ok(())
            }
        }
    }

    pub fn realloc(&self, space_to_add: usize, payer: &Signer<'info>, system_program: &Program<'info, System>) -> Result<()> {
        msg!("Reallocating account size to add user to Season");
        let account_info = self.vinci_quiz.to_account_info();
        let new_account_size = account_info.data_len() + space_to_add;

        // Determine additional rent required
        let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
        let additional_rent_to_fund = lamports_required - account_info.lamports();
        
        //To be deleted
        msg!("Adding a new player has the cost of {:?} (0.001572) SOL", additional_rent_to_fund);

        // Perform transfer of additional rent
        let cpi_program = system_program.to_account_info();
        let cpi_accounts = system_program::Transfer{
            from: payer.to_account_info(), 
            to: account_info.clone(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        system_program::transfer(cpi_context,additional_rent_to_fund)?;

        // Reallocate the account
        account_info.realloc(new_account_size, false)?;
        msg!("Account Size Updated");

        Ok(())
    }
}