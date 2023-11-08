use crate::*;

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, seeds = [b"VinciWorldAccount1", user.key().as_ref()], bump, payer = user, space = 8 + 8 + 8 + 32 + 1 + 1 + (4 + UserDetails::INIT_SPACE * 10))]
    pub base_account: Account<'info, BaseAccount>,
    pub system_program: Program<'info, System>
}

impl<'info> StartStuffOff<'info> {
    pub fn start_stuff_off(&mut self) -> Result<()> {
        let pubkey = Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap();
        let base_account = &mut self.base_account;
        let result = base_account.key();
        msg!(&result.to_string());
        base_account.total_amount = 0;
        base_account.score = 0;
        base_account.authority = pubkey;
        base_account.level = 1;

        Ok(())
    }
}

#[account]
pub struct BaseAccount {
    pub total_amount: u64,
    pub score: u64,
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
