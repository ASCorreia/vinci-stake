use crate::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct StakeCtx<'info>{
    //TBD Validate StakeEntry and StakePool seed through anchor macros
    #[account(mut, seeds = [b"VinciWorldStakeEntry_28", user.key().as_ref()], bump = stake_entry.bump, constraint = stake_entry.pool == stake_pool.key() @ CustomError::InvalidStakePool)]
    pub stake_entry: Box<Account<'info, StakeEntry>>,
    #[account(mut)]
    pub stake_pool: Box<Account<'info, StakePool>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub original_mint: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,

    #[account(mut)]
    pub from_mint_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to_mint_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub test: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: AccountInfo<'info>,
}

impl<'info> StakeCtx<'info> {
    pub fn stake_custodial(&mut self) -> Result<()> {

        let original_mint = self.stake_entry.original_mint.key();

        //TBD Do checks to the stake accounts (pool settings) and add more custom errors

        // Transfer NFT
        let cpi_accounts = token::Transfer {
            from: self.from_mint_token_account.to_account_info(),
            to: self.to_mint_token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_context, 1)?;

        //self.stake_entry.original_owner = self.from_mint_token_account.key(); -> Probably not needed
        //self.stake_entry.staking_owner = self.to_mint_token_account.key(); -> Probably not needed

        //Set the last staked time
        self.stake_entry.last_staked_at = Clock::get().unwrap().unix_timestamp;
        self.stake_entry.last_updated_at = Some(Clock::get().unwrap().unix_timestamp);
        
        //Update the total staked time
        self.stake_entry.total_stake_seconds = self.stake_entry.total_stake_seconds.saturating_add(
            (u128::try_from(Clock::get().unwrap().unix_timestamp).unwrap())
                .saturating_sub(u128::try_from(self.stake_entry.last_staked_at).unwrap()),
        );

        //Flag that the original mint has been claimed by the pool
        self.stake_entry.original_mint_claimed.push(original_mint);

        /* The following is an approach to store staking time if we decide to have multiple mints per entry */
        let staking_time = StakeTime{time: self.stake_entry.total_stake_seconds, mint: original_mint};
        self.stake_entry.original_mint_seconds_struct.push(staking_time);
        /* ------------------------------------------------------------------------------------------------ */

        self.stake_pool.total_staked += 1;
        self.stake_entry.amount += 1;
        
        Ok(())
    }

    pub fn stake_non_custodial(&mut self) -> Result<()> {
        let pda = &mut self.test;
        let stake_pool = &mut self.stake_pool;
        let stake_entry = &mut self.stake_entry;

        let original_mint = &mut self.original_mint;

        let user_token_accout = &mut self.from_mint_token_account;
        let program_id = &mut self.token_program;

        let token_edition = &mut self.master_edition;

        let authority = &mut self.user;

        let program_metadata_id = &mut self.token_metadata_program;

        msg!("stake_pool: {:?}", stake_pool.to_account_info().key);
        msg!("stake_entry: {:?}", stake_entry.to_account_info().key);
        msg!("original_mint: {:?}", original_mint.key);
        msg!("user_token_account: {:?}", user_token_accout.to_account_info().key);
        msg!("authority: {:?}", authority.key);
        msg!("token_program: {:?}", program_id.to_account_info().key);
        msg!("master_edition: {:?}", token_edition.to_account_info().key);
        
        let cpi_accounts = token::Approve {
            to: user_token_accout.to_account_info(),
            delegate: stake_entry.to_account_info(),
            authority: authority.to_account_info(),
        };
        let cpi_program = program_id.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        token::approve(cpi_context, 1)?;

        // Define the seeds
        let (pda_address, pda_bump) = Pubkey::find_program_address(&[b"VinciWorldStakeEntry_28", pda.key().as_ref()], &id());
        msg!("Derived PDA Address: {}", pda_address);
        msg!("Derived PDA Bump: {}", pda_bump);

        let seeds = &[
            "VinciWorldStakeEntry_28".as_bytes(),
            &pda.key().clone().to_bytes(),
            &[pda_bump]
        ];

        invoke_signed(
            &freeze_delegated_account(
                program_metadata_id.key(),
                stake_entry.key(),
                user_token_accout.key(),
                token_edition.key(),
                original_mint.key(),
            ),
            &[
                stake_entry.to_account_info(),
                user_token_accout.to_account_info(),
                token_edition.to_account_info(),
                original_mint.to_account_info(),
            ],
            &[seeds], //&[&[b"VinciWorldStakeEntry_28", pda.key().as_ref(), &[pda_bump]]],
        )?;

        /*TBD:
        Either use invoke_signed or try to use the program itself as delegation authority
        Try creating a PDA with a predefined seed (PDA_WORD for instance) and sign with that account (Create a dedicated PDA for this purpose))
        */
        
        //stake_entry.original_owner = user_token_accout.key(); -> Probably not needed
        //stake_entry.staking_owner = stake_entry.key(); -> Probably not needed

        //Set the last staked time
        stake_entry.last_staked_at = Clock::get().unwrap().unix_timestamp;
        
        //Update the total staked time
        stake_entry.total_stake_seconds = stake_entry.total_stake_seconds.saturating_add(
            (u128::try_from(Clock::get().unwrap().unix_timestamp).unwrap())
                .saturating_sub(u128::try_from(stake_entry.last_staked_at).unwrap()),
        );

        //Flag that the original mint has been claimed by the pool
        stake_entry.original_mint_claimed.push(original_mint.key());

        /* The following is an approach to store staking time if we decide to have multiple mints per entry */
        let staking_time = StakeTime{time: stake_entry.total_stake_seconds, mint: original_mint.key()};
        stake_entry.original_mint_seconds_struct.push(staking_time);
        /* ------------------------------------------------------------------------------------------------ */

        stake_pool.total_staked += 1;
        stake_entry.amount += 1;

        Ok(())
    }
}