use crate::*;

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, seeds = [b"VinciWorldAccount", user.key().as_ref()], bump, payer = user, space = 3500)]
    pub base_account: Account<'info, BaseAccount>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: UncheckedAccount<'info>, //Token Account (Represents the token)
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>, //Destination of the mint. The token that we want to send to tokens to!
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: UncheckedAccount<'info> //Authority to mint the token
}

#[derive(Accounts)]
pub struct BurnToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: UncheckedAccount<'info>, //Token Account (Represents the token)
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>, //Destination of the mint. The token that we want to send tokens to!
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: UncheckedAccount<'info> //Authority to mint the token
}

#[derive(Accounts)]
pub struct AddAmount<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveAmmount<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub ammount: String,
    pub user_address: Pubkey
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: UncheckedAccount<'info>, //Token Account (Represents the token)
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>, //Destination of the mint. The token that we want to send to tokens to!
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: UncheckedAccount<'info>, //Authority to mint the token (Shall be the Signer as well)
}

#[account]
pub struct BaseAccount {
    pub total_amount: u64,
    pub owner: Pubkey,
    pub spare_struct: Vec<ItemStruct>
}