use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("5wvAnEqxro6JLFTkCTHtCnd4daWjPpEkDiK7HgrUEZcd");

pub mod contexts;

pub use contexts::*;

#[program]
pub mod vinci_quiz {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vinci_quiz.tournament = vec![];
        ctx.accounts.vinci_quiz.entries = 0;
        ctx.accounts.vinci_quiz.bump = *ctx.bumps.get("vinci_quiz").unwrap();

        msg!("Vinci Quiz Season Initialized");

        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>) -> Result<()> {
        ctx.accounts.add_player()?;
        ctx.accounts.vinci_quiz.order_entries()?;

        Ok(())
    }

    pub fn update_score(ctx: Context<UpdateScore>, correct: bool) -> Result<()> {
        require!(ctx.accounts.authority.key() == Pubkey::from_str("7qZkw6j9o16kqGugWTj4u8Lq9YHcPAX8dgwjjd9EYrhQ").unwrap(), CustomError::InvalidAuthority);
        
        match correct {
            true => ctx.accounts.update_score(30)?,
            false => {
                let xorshift = ctx.accounts.xorshift64star(Clock::get()?.slot);
                let rand = xorshift % 20;
                msg!("Random number generated is {:?}", rand);
                ctx.accounts.update_score(rand as u32)?;
            }
        }

        ctx.accounts.vinci_quiz.order_entries()?;

        Ok(())
    }

    pub fn upgrade(ctx: Context<Upgrade>) -> Result<()> {
        ctx.accounts.upgrade()?;

        Ok(())
    }

    pub fn mega_upgrade(ctx: Context<MegaUpgrade>, creator_key: Pubkey, uri: String, title: String) -> Result<()> {
        ctx.accounts.mega_upgrade(creator_key, uri, title)?;

        Ok(())
    }

    pub fn season_rewards(ctx: Context<SeasonRewards>) -> Result<()> {
        require!(ctx.accounts.authority.key() == Pubkey::from_str("7qZkw6j9o16kqGugWTj4u8Lq9YHcPAX8dgwjjd9EYrhQ").unwrap(), CustomError::InvalidAuthority);
        
        ctx.accounts.distribute_rewards()?;

        Ok(())
    }

    pub fn close_season(ctx: Context<CloseSeason>) -> Result<()> {
        ctx.accounts.close_season()?;

        Ok(())
    }
}
