use crate::*;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = destination)]
    vinci_swap: Account<'info, VinciSwap>,
    ///CHECK: This not dangerous as it will only receive the lamports from the account closure
    #[account(mut)]
    destination: AccountInfo<'info>,
}

impl<'info> Close<'info> {
    pub fn close_pool(&mut self) -> Result<()> {
        msg!("Liquidity Pool closed!");

        Ok(())
    }
}