use crate::*;

#[account]
pub struct BaseAccount {
    pub total_amount: u64,
    //To be directly used in season / tournament
    //pub score: u64,
    pub authority: Pubkey,
    pub bump: u8,
    pub level: u8,
    pub spare_struct: Vec<UserDetails>
}

#[derive(InitSpace, Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UserDetails {
    pub ammount: u32,
    pub user_address: Pubkey
}

/* --------------- TBD -------------- */
// Change 'owner' to 'authority'