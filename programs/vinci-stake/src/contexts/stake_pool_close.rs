use crate::*;

#[derive(Accounts)]
pub struct ClosePool<'info> {
    #[account(mut, close = destination)]
    stake_pool: Account<'info, StakePool>,
    #[account(mut)]
    destination: SystemAccount<'info>,
}

impl<'info> ClosePool<'info> {
    pub fn close_stake_pool(&mut self) -> Result<()> {
        msg!("Stake Pool closed!");

        Ok(())
    }
}