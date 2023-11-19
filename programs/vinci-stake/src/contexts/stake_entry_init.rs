use crate::*;

#[derive(Accounts)]
pub struct InitializeStakeEntry<'info> {
    #[account(init, seeds = [b"VinciStakeEntry", user.key().as_ref()], bump, payer = user, 
        space = 8 + 32 + 8 + 8 + 16 + (4 + StakeTime::INIT_SPACE * 10) + (1 + 8) + (1 + 8) + 1 + 1)]
    pub stake_entry: Box<Account<'info, StakeEntry>>,
    #[account(mut)]
    pub stake_pool_account: Box<Account<'info, StakePool>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeStakeEntry<'info> {
    pub fn initialize(&mut self) -> Result<()> {
        
        self.stake_entry.pool = self.stake_pool_account.key();
        self.stake_entry.amount = 0;
        self.stake_entry.original_mint_seconds_struct = Vec::new();
        self.stake_entry.cooldown_start_seconds = None;
        self.stake_entry.misc = 0;
        
        Ok(())
    }
}