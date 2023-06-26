use std::ops::{Div, Mul};

use anchor_spl::{token::{Mint, TokenAccount, Token}, associated_token::AssociatedToken};

use crate::*;

#[derive(Accounts)]
pub struct Swap<'info> {
    //Liquidity Pool / Swap Program account
    #[account(seeds = [b"VinciSwap"], bump = vinci_swap.bump)]
    pub vinci_swap: Box<Account<'info, VinciSwap>>,

    //Mint that the user wants to receive in exchange
    #[account(constraint = user_receive_mint.key() != user_pay_mint.key() @ CustomError::TryingToSwapSameAssets)]
    pub user_receive_mint: Box<Account<'info, Mint>>,
    //Liquidity pool from where the tokens will be sent to the user
    #[account(mut, associated_token::mint = user_receive_mint, associated_token::authority = vinci_swap)]
    pub pool_receive_token_account: Box<Account<'info, TokenAccount>>,
    //User token account to where the tokens will be sent to
    #[account(init_if_needed, payer = user, associated_token::mint = user_receive_mint, associated_token::authority = user)] //We might want to init the user receive ATA
    pub user_receive_token_account: Box<Account<'info, TokenAccount>>,

    //Mint that the user wants to pay for the exchange
    pub user_pay_mint: Box<Account<'info, Mint>>,
    //Liquidity pool to where the user tokens will be sent to
    #[account(mut, associated_token::mint = user_pay_mint, associated_token::authority = vinci_swap)]
    pub pool_pay_token_account: Box<Account<'info, TokenAccount>>,
    //User token account to where the tokens will be sent to
    #[account(mut, associated_token:: mint = user_pay_mint, associated_token::authority = user)]
    pub user_pay_token_account: Box<Account<'info, TokenAccount>>,

    //The user account as signer
    #[account(mut)]
    pub user: Signer<'info>,

    //Token program (In order to swap tokens between accounts)
    pub token_program: Program<'info, Token>,
    //System to program (In order to swap SOL)
    pub system_program: Program<'info, System>,
    //Associated token program not needed as we are not initializing any associated token account (we might actually need it for user_receive_token_account)
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Swap<'info> {
    pub fn calc_amount(&self, pool_receive_balance: u64, receive_decimals: u8, pool_pay_balance: u64, pay_decimals: u8, pay_amount: u64) -> Result<u64> {
        // Convert all values to nominal floats using their respective mint decimal
        let big_r = Swap::convert_to_float(pool_receive_balance, receive_decimals);
        let big_p = Swap::convert_to_float(pool_pay_balance, pay_decimals);
        let p = Swap::convert_to_float(pay_amount, pay_decimals);
        // Calculate `f(p)` to get `r`
        let bigr_times_p = big_r.mul(p);
        let bigp_plus_p = std::ops::Add::add(big_p, p);
        let r = bigr_times_p.div(bigp_plus_p);
        // Make sure `r` does not exceed liquidity
        if r > big_r {
            return Err(CustomError::NotEnoughLiquidity.into());
        }
        
        // Return the real value of `r`
        let res = Swap::convert_from_float(r, receive_decimals);
        Ok(res)
    }

    /// Converts a `u64` value - in this case the balance of a token account - into
    /// an `f32` by using the `decimals` value of its associated mint to get the
    /// nominal quantity of a mint stored in that token account
    ///
    /// For example, a token account with a balance of 10,500 for a mint with 3
    /// decimals would have a nominal balance of 10.5
    pub fn convert_to_float(value: u64, decimals: u8) -> f32 {
        (value as f32).div(f32::powf(10.0, decimals as f32))
    }

    /// Converts a nominal value - in this case the calculated value `r` - into a
    /// `u64` by using the `decimals` value of its associated mint to get the real
    /// quantity of the mint that the user will receive
    ///
    /// For example, if `r` is calculated to be 10.5, the real amount of the asset
    /// to be received by the user is 10,500
    pub fn convert_from_float(value: f32, decimals: u8) -> u64 {
        value.mul(f32::powf(10.0, decimals as f32)) as u64
    }
}