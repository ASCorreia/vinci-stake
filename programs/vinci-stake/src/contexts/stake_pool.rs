use crate::*;

#[derive(Accounts)]
pub struct InitializeStakePool<'info> {
    #[account(init, seeds = [b"VinciWorldStakePool_28", user.key().as_ref()], bump, payer = user, space = 3500)]
    pub stake_pool: Box<Account <'info, StakePool>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeStakePool<'info> {
    pub fn intialize(&mut self) -> Result<()> {

        self.stake_pool.double_or_reset_enabled = None;
        self.stake_pool.cooldown_seconds = None;
        self.stake_pool.identifier = 0xBEBACAFE;
        self.stake_pool.requires_authorization = false;
        self.stake_pool.requires_creators.push(Pubkey::from_str("7qZkw6j9o16kqGugWTj4u8Lq9YHcPAX8dgwjjd9EYrhQ").unwrap());
        self.stake_pool.max_stake_amount = None;
        self.stake_pool.total_staked = 0;
        
        Ok(())
    }
}

//Update this struct by removing unnecessary fields
#[account]
pub struct StakePool {
    pub identifier: u64,
    pub authority: Pubkey,
    pub requires_creators: Vec<Pubkey>,
    pub requires_collections: Vec<Pubkey>,
    pub requires_authorization: bool,
    pub overlay_text: String,
    pub image_uri: String,
    pub reset_on_stake: bool,
    pub total_staked: u32,
    pub cooldown_seconds: Option<u32>,
    pub min_stake_seconds: Option<u32>,
    pub end_date: Option<i64>,
    pub double_or_reset_enabled: Option<bool>,
    pub max_stake_amount: Option<u32>,
}