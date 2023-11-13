use crate::*;

#[account]
pub struct Tournament {
    pub owner: Pubkey,
    pub tournament_list: Vec<TournamentStruct>,
    pub prize_pool: u32,
}