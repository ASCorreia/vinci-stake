use crate::*;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = destination)]
    vinci_swap: Account<'info, VinciSwap>,
    ///CHECK: This not dangerous as it will only receive the lamports from the account closure
    #[account(mut)]
    destination: AccountInfo<'info>,
}