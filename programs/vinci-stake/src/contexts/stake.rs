use crate::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct StakeCtx<'info>{
    //TBD Validate StakeEntry and StakePool seed through anchor macros
    #[account(mut, seeds = [b"VinciStakeEntry", user.key().as_ref()], bump = stake_entry.bump, constraint = stake_entry.pool == stake_pool.key() @ CustomError::InvalidStakePool)]
    pub stake_entry: Box<Account<'info, StakeEntry>>,
    #[account(mut, seeds = [b"VinciStakePool"], bump = stake_pool.bump)]
    pub stake_pool: Box<Account<'info, StakePool>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub original_mint: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,

    #[account(mut, constraint = to_mint_token_account.mint == original_mint.key() @ CustomError::InvalidMint)]
    pub from_mint_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut, constraint = to_mint_token_account.mint == original_mint.key() @ CustomError::InvalidMint)]
    pub to_mint_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: AccountInfo<'info>,
}

impl<'info> StakeCtx<'info> {
    pub fn stake_custodial(&mut self) -> Result<()> {
        if self.stake_entry.amount >= 10 {
            self.realloc(StakeTime::INIT_SPACE, &self.user, &self.system_program)?;
        }

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

        //Set the last staked time
        self.stake_entry.last_staked_at = Clock::get().unwrap().unix_timestamp;
        self.stake_entry.last_updated_at = Some(Clock::get().unwrap().unix_timestamp);
        
        //Update the total staked time
        self.stake_entry.total_stake_seconds = self.stake_entry.total_stake_seconds.saturating_add(
            (u128::try_from(Clock::get().unwrap().unix_timestamp).unwrap())
                .saturating_sub(u128::try_from(self.stake_entry.last_staked_at).unwrap()),
        );

        /* The following is an approach to store staking time if we decide to have multiple mints per entry */
        let staking_time = StakeTime{time: self.stake_entry.total_stake_seconds, mint: original_mint};
        self.stake_entry.original_mint_seconds_struct.push(staking_time);
        /* ------------------------------------------------------------------------------------------------ */

        self.stake_pool.total_staked += 1;
        self.stake_entry.amount += 1;
        
        Ok(())
    }

    pub fn stake_non_custodial(&mut self) -> Result<()> {
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
        let (pda_address, pda_bump) = Pubkey::find_program_address(&[b"VinciStakeEntry", authority.key().as_ref()], &id());
        msg!("Derived PDA Address: {}", pda_address);
        msg!("Derived PDA Bump: {}", pda_bump);

        let seeds = &[
            "VinciStakeEntry".as_bytes(),
            &authority.key().clone().to_bytes(),
            &[stake_entry.bump]
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
            &[seeds], //&[&[b"VinciStakeEntry", pda.key().as_ref(), &[pda_bump]]],
        )?;

        /*TBD:
        Either use invoke_signed or try to use the program itself as delegation authority
        Try creating a PDA with a predefined seed (PDA_WORD for instance) and sign with that account (Create a dedicated PDA for this purpose))
        */

        //Set the last staked time
        stake_entry.last_staked_at = Clock::get().unwrap().unix_timestamp;
        
        //Update the total staked time
        stake_entry.total_stake_seconds = stake_entry.total_stake_seconds.saturating_add(
            (u128::try_from(Clock::get().unwrap().unix_timestamp).unwrap())
                .saturating_sub(u128::try_from(stake_entry.last_staked_at).unwrap()),
        );

        /* The following is an approach to store staking time if we decide to have multiple mints per entry */
        let staking_time = StakeTime{time: stake_entry.total_stake_seconds, mint: original_mint.key()};
        stake_entry.original_mint_seconds_struct.push(staking_time);
        /* ------------------------------------------------------------------------------------------------ */

        stake_pool.total_staked += 1;
        stake_entry.amount += 1;

        Ok(())
    }

    pub fn realloc(&self, space_to_add: usize, payer: &Signer<'info>, system_program: &Program<'info, System>) -> Result<()> {
        msg!("Reallocating account size to add new mint to Stake Entry");
        let account_info = self.stake_entry.to_account_info();
        let new_account_size = account_info.data_len() + space_to_add;

        // Determine additional rent required
        let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
        let additional_rent_to_fund = lamports_required - account_info.lamports();

        // Perform transfer of additional rent
        let cpi_program = system_program.to_account_info();
        let cpi_accounts = system_program::Transfer{
            from: payer.to_account_info(), 
            to: account_info.clone(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        system_program::transfer(cpi_context,additional_rent_to_fund)?;

        // Reallocate the account
        account_info.realloc(new_account_size, false)?;
        msg!("Account Size Updated");

        Ok(())
    }
}