use crate::*;

#[derive(Accounts)]
pub struct InitializeStakeEntry<'info> {
    #[account(init, seeds = [b"VinciStakeEntry", user.key().as_ref()], bump, payer = user, 
        space = 8 + 32 + 8 + 32 + 8 + 16 + (4 + StakeTime::INIT_SPACE * 10) + (1 + 8) + (1 + 8) + 1)]
    pub stake_entry: Box<Account<'info, StakeEntry>>,
    #[account(mut)]
    pub stake_pool_account: Box<Account<'info, StakePool>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub original_mint: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub original_mint_metadata: AccountInfo<'info>,

    //original_mint and original_mint_metadata should be transfered to the stake context??

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeStakeEntry<'info> {
    pub fn initialize(&mut self) -> Result<()> {
        
        self.stake_entry.original_mint = self.original_mint.key();
        self.stake_entry.pool = self.stake_pool_account.key();
        self.stake_entry.amount = 0; //Probably not needed
        self.stake_entry.original_mint_seconds_struct = Vec::new();
        self.stake_entry.cooldown_start_seconds = None;

        // All the checks below shouls be moved to the stake function? Since we are allowing multiple stakes per user
        // assert metadata account derivation (asserts from a programID, an account and a path (seeds))
        assert_derivation(
            &mpl_token_metadata::id(),
            &self.original_mint_metadata.to_account_info(),
            &[
                mpl_token_metadata::state::PREFIX.as_bytes(),
                mpl_token_metadata::id().as_ref(),
                self.original_mint.key().as_ref(),
            ],
        )?;

        require!(self.original_mint_metadata.data_is_empty() == false, CustomError::MetadataAccountEmpty);

        /* Borrow and deserialize the metada account from the original mint metadata */
        let mint_metadata_data = self.original_mint_metadata.try_borrow_mut_data().expect("Error borrowing data");
        require!(self.original_mint_metadata.to_account_info().owner.key() == mpl_token_metadata::id(), CustomError::InvalidMintOwner); //Checks that the owner is the Metadadata program
        let original_mint_metadata = Metadata::deserialize(&mut mint_metadata_data.as_ref()).expect("Error deserializng metadata");
        require!(original_mint_metadata.mint == self.original_mint.key(), CustomError::InvalidMint); //Checks that both the original mint and the one stored in the account are the same

        //Get the creators from the metadata and see if the it contains the ones required by the stake pool
        let creators = original_mint_metadata.data.creators.unwrap();
        //let collection = original_mint_metadata.collection.unwrap();
        let find_creators = creators.iter().find(|creator| self.stake_pool_account.requires_creators.contains(&creator.address) && !creator.verified); // (!)creator.verified

        //Checks that the creators have been found
        require!(find_creators.is_some() == true, CustomError::MissingCreators);
        
        Ok(())
    }
}

#[account]
pub struct StakeEntry {
    pub pool: Pubkey,
    pub amount: u64,
    pub original_mint: Pubkey,
    pub last_staked_at: i64, //Needed?? Can the last_updated_at be used for this?
    pub total_stake_seconds: u128,
    pub original_mint_seconds_struct: Vec<StakeTime>, //To be discussed as an approach to store mint time (if only one stake entry is used per user)
    pub cooldown_start_seconds: Option<i64>, //To be removed as is not being used? Or leave it as provision?
    pub last_updated_at: Option<i64>,
    pub bump: u8,
}

#[derive(InitSpace, Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StakeTime {
    pub time: u128,
    pub mint: Pubkey,
}