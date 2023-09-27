use vinci_accounts::{program::VinciAccounts, BaseAccount};

use crate::*;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub stake_entry: Box<Account<'info, StakeEntry>>,
    #[account(mut)]
    pub vinci_account: Box<Account<'info, BaseAccount>>,
    #[account(mut)]
    pub owner: Signer<'info>,

    pub accounts_program: Program<'info, VinciAccounts>,
    pub rewards_program: Program<'info, VinciRewards>,
}

impl<'info> ClaimRewards<'info> {
    pub fn claim_rewards(&mut self) -> Result<()> {
        let stake_entry = &mut self.stake_entry;

        let cpi_program = self.rewards_program.to_account_info();
        let cpi_accounts = vinci_rewards::cpi::accounts::Initialize{
            stake_entry: stake_entry.to_account_info(),
            vinci_account: self.vinci_account.to_account_info(),
            accounts_program: self.accounts_program.to_account_info(),
            owner: self.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        vinci_rewards::cpi::initialize(cpi_ctx)?;

        // Since the rewards have been claimed, set the stacked time of each mint to 0
        for index in 0..stake_entry.original_mint_seconds_struct.len() {
            stake_entry.original_mint_seconds_struct[index].time = 0;
        }

        Ok(())
    }
}