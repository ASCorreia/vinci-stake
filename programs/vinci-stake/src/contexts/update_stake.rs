use crate::*;

#[derive(Accounts)]
pub struct UpdateStakeCtx<'info>{
    pub stake_pool: Signer<'info>,
}