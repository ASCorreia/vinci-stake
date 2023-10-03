use crate::*;

#[derive(Accounts)]
pub struct UpdateStakeCtx<'info>{
    #[account(mut, seeds = [b"VinciStakeEntry", stake_pool.key().as_ref()], bump = stake_entry.bump)]
    pub stake_entry: Account<'info, StakeEntry>,
    pub stake_pool: Signer<'info>,
}

impl<'info> UpdateStakeCtx<'info> {
    pub fn update_stake(&mut self) -> Result<()> {
        //Update the stake time
        let mut total_stake_seconds: u128;
        for index in 0..self.stake_entry.original_mint_seconds_struct.len() {
            total_stake_seconds = self.stake_entry.original_mint_seconds_struct[index].time + (self.stake_entry.total_stake_seconds.saturating_add(
                (u128::try_from(Clock::get().unwrap().unix_timestamp).unwrap())
                    .saturating_sub(u128::try_from(self.stake_entry.last_staked_at).unwrap()),
            ));

            self.stake_entry.original_mint_seconds_struct[index].time = total_stake_seconds;
        }

        //Set the last staked time
        self.stake_entry.last_staked_at = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }
}