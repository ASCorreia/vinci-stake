use crate::*;

#[derive(Accounts)]
pub struct CloseEntry<'info> {
    #[account(mut, close = destination)]
    stake_entry: Account<'info, StakeEntry>,
    #[account(mut)]
    destination: SystemAccount<'info>,
}

impl<'info> CloseEntry<'info> {
    pub fn close_stake_entry(&mut self) -> Result<()> {
        msg!("Stake Entry closed!");

        Ok(())
    }
}