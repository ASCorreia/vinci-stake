use anchor_lang::prelude::*;

declare_id!("GeXG8abTTediTsezVsBGBuxRWuZ15wK2UDd45dQ3vQKq");

pub mod contexts;

pub use contexts::*;

#[program]
pub mod vinci_swap {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vinci_account = &mut ctx.accounts.vinci_swap;

        vinci_account.assets = vec![];
        vinci_account.bump = *ctx.bumps.get("vinci_swap").unwrap();

        msg!("The initialized bump is {:?}", vinci_account.bump);
        Ok(())
    }

    pub fn add_token(ctx: Context<AddToken>) -> Result<()> {
        //let payer = &mut ctx.accounts.payer;
        //let system_program = &mut ctx.accounts.system_program;

        ctx.accounts.check_token()?;

        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_to_pool(amount)?;

        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        //Check swap amounts
        require!(amount != 0, CustomError::InvalidAmount);
        
        //Details of the token requested by the user:
        //Mint, From, To
        let (user_receive_mint, pool_receive_token_account, user_receive_token_account) = 
            (&ctx.accounts.user_receive_mint,
            &ctx.accounts.pool_receive_token_account, 
            &ctx.accounts.user_receive_token_account);

        //Details of the mint the user is paying for the swap
        //Mint, From, To
        let (user_pay_mint, user_pay_token_account, pool_pay_token_account) = 
            (&ctx.accounts.user_pay_mint,
            &ctx.accounts.user_pay_token_account,
            &ctx.accounts.pool_pay_token_account);

        //Check that the pools are valid
        let receive_pool_found = ctx.accounts.vinci_swap.assets.iter().find(|&&mint| mint == user_receive_mint.key()).is_some();
        let pay_pool_found = ctx.accounts.vinci_swap.assets.iter().find(|&&mint| mint == user_pay_mint.key()).is_some();
        require!(receive_pool_found == true, CustomError::NoPoolFound);
        require!(pay_pool_found == true, CustomError::NoPoolFound);

        //Calculate the amount to send to the user, based on pool liquidity
        let amount_to_send = ctx.accounts.calc_amount(pool_receive_token_account.amount, user_receive_mint.decimals, 
            pool_pay_token_account.amount, user_pay_mint.decimals, amount)?;

        //Send amounts to and from pool / user
        ctx.accounts.vinci_swap.transfer_to_pool(user_pay_token_account, pool_pay_token_account, amount, &ctx.accounts.user, &ctx.accounts.token_program)?;
        ctx.accounts.vinci_swap.transfer_from_pool(pool_receive_token_account, user_receive_token_account, amount_to_send, &ctx.accounts.token_program)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close_pool()?;
        
        Ok(())
    }
}

//update the account to be a PDA, and fetch the bump directly (dont have it as an argument) - Done
//Implement the swap functionality according to liquidity algorithm and test it with swaps between both pools - Done
