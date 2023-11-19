use crate::*;

#[account] //consider adding InitSpace and giving fixed Vec Size
pub struct StakeEntry {
    pub pool: Pubkey,
    pub amount: u64,
    pub last_staked_at: i64, //Needed?? Can the last_updated_at be used for this?
    pub total_stake_seconds: u128,
    pub original_mint_seconds_struct: Vec<StakeTime>, //To be discussed as an approach to store mint time (if only one stake entry is used per user)
    pub cooldown_start_seconds: Option<i64>, //To be removed as is not being used? Or leave it as provision?
    pub last_updated_at: Option<i64>,
    pub bump: u8,
    pub misc: u8,
}

#[derive(InitSpace, Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StakeTime {
    pub time: u128,
    pub mint: Pubkey,
}