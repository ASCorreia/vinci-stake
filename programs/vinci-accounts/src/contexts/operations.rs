use crate::*;

#[derive(Accounts)]
pub struct Operations<'info> {
    #[account(mut, seeds = [b"VinciWorldAccount1", owner.key().as_ref()], bump = base_account.bump)] //bump issue with CPI from Vinci Rewards
    pub base_account: Account<'info, BaseAccount>,
    ///CHECK: This is not dangerous
    #[account(mut)]
    pub owner: AccountInfo<'info>, //Consider Signer
}

impl<'info> Operations<'info> {
    pub fn add_amount(&mut self, amount: u64) -> Result<()> {
        //Security check to be used when the frontend / backend is updated 
        //require!(ctx.accounts.owner.is_signer == true && *ctx.accounts.owner.key == base_account.owner, CustomError::WrongSigner);

        self.base_account.total_amount += amount;

        Ok(())
    }

    pub fn remove_amount(&mut self, amount: u64) -> Result<()> {
        require!(amount <= self.base_account.total_amount, CustomError::InsufficientBalanceSpl);
        require!(self.owner.is_signer == true && *self.owner.key == self.base_account.authority, CustomError::WrongSigner);
        
        self.base_account.total_amount -= amount;
        Ok(())
    }

    pub fn set_score(&mut self, score: u64) -> Result<()> {
        //Security check to be used when the frontend / backend is updated 
        //require!(ctx.accounts.owner.is_signer == true && *ctx.accounts.owner.key == base_account.owner, CustomError::WrongSigner);

        msg!("Update the set score function to the season itself");
        //self.base_account.score = score;

        Ok(())
    }
}

