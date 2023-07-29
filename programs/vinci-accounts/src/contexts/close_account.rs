use crate::*;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = destination)]
    vinci_account: Account<'info, BaseAccount>,
    ///CHECK: This not dangerous as it will only receive the lamports from the account closure
    #[account(mut)]
    destination: AccountInfo<'info>,
}