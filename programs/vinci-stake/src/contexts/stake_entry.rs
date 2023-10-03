use crate::*;

#[derive(Accounts)]
pub struct InitializeStakeEntry<'info> {
    #[account(init, seeds = [b"VinciStakeEntry", user.key().as_ref()], bump, payer = user, 
        space = 8 + 32 + 8 + 8 + 16 + (4 + StakeTime::INIT_SPACE * 10) + (1 + 8) + (1 + 8) + 1)]
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
        self.stake_entry.amount = 0; //Probably not needed
        self.stake_entry.original_mint_seconds_struct = Vec::new();
        self.stake_entry.cooldown_start_seconds = None;
        
        Ok(())
    }
}

#[account]
pub struct StakeEntry {
    pub pool: Pubkey,
    pub amount: u64,
    pub last_staked_at: i64, //Needed?? Can the last_updated_at be used for this?
    pub total_stake_seconds: u128,
    pub original_mint_seconds_struct: Vec<StakeTime>, //To be discussed as an approach to store mint time (if only one stake entry is used per user)
    pub cooldown_start_seconds: Option<i64>, //To be removed as is not being used? Or leave it as provision?
    pub last_updated_at: Option<i64>,
    pub bump: u8,
}

#[derive(InitSpace, Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StakeTime {
    pub time: u128,
    pub mint: Pubkey,
}