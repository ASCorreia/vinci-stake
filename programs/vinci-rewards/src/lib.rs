use anchor_lang::prelude::*;

declare_id!("4DszCYyUCeXYX3qRQoTdWscvXUyMkGmAm7KMrgLYX4FF");

pub mod contexts;

pub use contexts::*;

#[program]
pub mod vinci_rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let stake_entry = &mut ctx.accounts.stake_entry.try_borrow_mut_data()?;

        let entry_deserialized = StakeEntry::try_deserialize(&mut stake_entry.as_ref()).expect("Error deserializing stake entry");

        let total_staked: u128 = entry_deserialized.original_mint_seconds_struct.iter().map(|entry| entry.time).sum();
        msg!("Total staked time: {:?}", total_staked);

        let cpi_program = ctx.accounts.accounts_program.to_account_info();
        let cpi_accounts = vinci_accounts::cpi::accounts::AddAmount {
            base_account: ctx.accounts.vinci_account.to_account_info(),
            owner: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI created to interact to Vinci Accounts Program: {:?}", cpi_ctx.program.key());

        vinci_accounts::cpi::add_ammount(cpi_ctx, 50)?;
        msg!("Call to Vinci Accounts program succesfull");

        Ok(())
    }
}

// ----- Next Steps ----- //
/*
    1. Vinci rewards will interact with vinci world main contract to update accounts - Amount of tokens, etc.
    2. Mint NFT function should be moved to rewards program?
 */
