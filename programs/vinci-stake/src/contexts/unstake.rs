use crate::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct UnstakeCtx<'info>{
    //TBD Validate StakeEntry and StakePool seed through anchor macros
    #[account(mut, seeds = [b"VinciStakeEntry", user.key().as_ref()], bump = stake_entry.bump, constraint = stake_entry.pool == stake_pool.key() @ CustomError::InvalidStakePool)]
    pub stake_entry: Box<Account<'info, StakeEntry>>,
    #[account(mut, seeds = [b"VinciStakePool1"], bump = stake_pool.bump)]
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
    pub token_metadata_program: AccountInfo<'info>,
}

impl<'info> UnstakeCtx<'info> {
    pub fn claim_custodial(&mut self) -> Result<()> {
        //let authority = Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap();

        let stake_pool = &mut self.stake_pool;
        let stake_entry = &mut self.stake_entry;

        let from_token_account = &mut self.from_mint_token_account;
        let to_token_account = &mut self.to_mint_token_account;

        //let original_mint = &mut self.original_mint;

        let signer = &mut self.user;

        //require!(stake_entry.original_mint_claimed.iter().find(|mint| **mint == original_mint.key()) == Some(&original_mint.key()), CustomError::OriginalMintNotClaimed);
        //require!(stake_entry.stake_mint_claimed.iter().find(|mint| **mint == original_mint.key()) == None, CustomError::MintAlreadyClaimed);
        //require!(signer.key() == authority, CustomError::UnauthorizedSigner);

        //Transfer NFT -> To be updated, needs to transfered in and out of a PDA
        let cpi_accounts = token::Transfer{
            from: from_token_account.to_account_info(),
            to: to_token_account.to_account_info(),
            authority: signer.to_account_info(),
        };       
        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_context, 1)?;

        //stake_entry.stake_mint_claimed.push(original_mint.key());
        //stake_entry.original_mint_claimed.retain(|mint| *mint != self.original_mint.key());
        /* The following is an approach to store staking time if we decide to have multiple mints per entry */
        stake_entry.original_mint_seconds_struct.retain(|stake_mint_struct| stake_mint_struct.mint != self.original_mint.key());
        /* ------------------------------------------------------------------------------------------------ */
        stake_entry.total_stake_seconds = 0;

        let _ = stake_pool.total_staked.saturating_sub(1); //-= 1;
        let _ = stake_entry.amount.saturating_sub(1); //-= 1;

        Ok(())
    }

    pub fn claim_non_custodial(&mut self) -> Result<()> {
        //let authority = Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap();
        let stake_pool = &mut self.stake_pool;
        let stake_entry = &mut self.stake_entry;

        let original_mint = &mut self.original_mint;

        let user_token_accout = &mut self.from_mint_token_account;

        let token_edition = &mut self.master_edition;

        let program_metadata_id = &mut self.token_metadata_program;

        require!(stake_entry.pool == stake_pool.key(), CustomError::InvalidStakePool);
        //require!(stake_entry.original_mint_claimed.iter().find(|mint| **mint == original_mint.key()) == Some(&original_mint.key()), CustomError::OriginalMintNotClaimed);
        //require!(stake_entry.stake_mint_claimed.iter().find(|mint| **mint == original_mint.key()) == None, CustomError::MintAlreadyClaimed);
        //require!(signer.key() == authority, CustomError::UnauthorizedSigner);

        require!(stake_entry.original_mint_seconds_struct.iter().find(
            |mint_struct|mint_struct.mint == original_mint.key()).is_some(), CustomError::MintAlreadyClaimed);

        // Define the seeds
        let (pda_address, pda_bump) = Pubkey::find_program_address(&[b"VinciStakeEntry", self.user.key().as_ref()], &id());
        msg!("Derived PDA Address: {}", pda_address);
        msg!("Derived PDA Bump: {}", pda_bump);

        let seeds = &[
            "VinciStakeEntry".as_bytes(),
            &self.user.key().clone().to_bytes(),
            &[stake_entry.bump]
        ];

        invoke_signed(
            &thaw_delegated_account(
                program_metadata_id.key(),
                stake_entry.key(),
                user_token_accout.key(),
                token_edition.key(),
                original_mint.key(),
            ),
            &[
                stake_entry.to_account_info(),//pda.to_account_info(),
                user_token_accout.to_account_info(),
                token_edition.to_account_info(),
                original_mint.to_account_info(),
            ],
            &[seeds], //&[&[b"VinciStakeEntry", pda.key().as_ref(), &[pda_bump]]],
        )?;

        /* The following is an approach to store staking time if we decide to have multiple mints per entry */
        stake_entry.original_mint_seconds_struct.retain(|stake_mint_struct| stake_mint_struct.mint != self.original_mint.key());
        /* ------------------------------------------------------------------------------------------------ */
        stake_entry.total_stake_seconds = 0;

        let _ = stake_pool.total_staked.saturating_sub(1); //-= 1;
        let _ = stake_entry.amount.saturating_sub(1); //-= 1;

        Ok(())
    }
}
