use anchor_spl::token::{Token, TokenAccount, Mint};

use crate::*;

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, seeds = [b"VinciWorldAccount1", user.key().as_ref()], bump, payer = user, space = 3500)]
    pub base_account: Account<'info, BaseAccount>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    pub token_program: Program<'info, Token>,
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: Account<'info, Mint>, //Token Account (Represents the token)
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, //Destination of the mint. The token that we want to send to tokens to!
    #[account(mut)]
    pub payer: Signer<'info> //Authority to mint the token
}

#[derive(Accounts)]
pub struct BurnToken<'info> {
    pub token_program: Program<'info, Token>,
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: Account<'info, Mint>, //Token Account (Represents the token)
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, //Destination of the mint. The token that we want to send tokens to!
    #[account(mut)]
    pub payer: Signer<'info> //Authority to mint the token
}

#[derive(Accounts)]
pub struct AddAmount<'info> {
    #[account(mut, seeds = [b"VinciWorldAccount1", owner.key().as_ref()], bump = base_account.bump)] //bump issue with CPI from Vinci Rewards
    pub base_account: Account<'info, BaseAccount>,
    ///CHECK: This is not dangerous
    #[account(mut)]
    pub owner: AccountInfo<'info>, //Consider Signer
}

#[derive(Accounts)]
pub struct RemoveAmmount<'info> {
    #[account(mut, seeds = [b"VinciWorldAccount1", owner.key().as_ref()], bump = base_account.bump)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    pub token_program: Program<'info, Token>,
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: Account<'info, Mint>, //Token Account (Represents the token)
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, //Destination of the mint. The token that we want to send to tokens to!
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //Authority to mint the token (Shall be the Signer as well)
}

#[account]
pub struct BaseAccount {
    pub total_amount: u64,
    pub owner: Pubkey,
    pub bump: u8,
    pub level: u8,
    pub spare_struct: Vec<ItemStruct>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub ammount: String,
    pub user_address: Pubkey
}

/* --------------- TBD -------------- */
// Change 'owner' to 'authority'
