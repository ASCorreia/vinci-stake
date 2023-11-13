use crate::*;

#[account]
pub struct QuizSeason {
    pub tournament: Vec<UserEntry>,
    pub entries: u32,
    pub bump: u8,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UserEntry {
    pub score: u32,
    pub level: u8,
    pub nft_minted: bool,
    pub user: Pubkey,
}