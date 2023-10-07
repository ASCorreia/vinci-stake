use crate::*;

#[derive(Accounts)]
pub struct StartTournament<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, seeds = [b"Tournament_1", user.key().as_ref()], bump, payer = user, space = 4000)]
    pub tournament: Account<'info, Tournament>,
    pub system_program: Program<'info, System>
}

impl<'info> StartTournament<'info> {
    pub fn start_tournament(&mut self, prize_pool: u32) -> Result<()> {
        let pubkey = Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap(); //to be updated with appropriate wallet PubKey

        let tournament = &mut self.tournament;
        tournament.tournament_list = Vec::new();
        tournament.owner = pubkey;
        tournament.prize_pool = prize_pool;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddPartcipant<'info> {
    pub user: Signer<'info>,
    pub tournament_list: Account<'info, Tournament>,
    pub new_participant: Account<'info, BaseAccount>,
}

impl<'info> AddPartcipant<'info> {
    pub fn add_tournament_participant(&mut self) -> Result<()> {
        let base_account = &mut self.new_participant;
        let tournament_list = &mut self.tournament_list;

        require!(self.user.is_signer == true && self.user.key() == tournament_list.owner, CustomError::WrongSigner);
        if !tournament_list.tournament_list.iter().any(|t| t.user == base_account.key()) {
            tournament_list.tournament_list.push(TournamentStruct { user: base_account.key(), score: 0 });
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PayTournament<'info> {
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct PayTournament2<'info> {
    pub tournament: Account<'info, Tournament>,
    pub owner: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct TournamentStruct {
    pub user: Pubkey,
    pub score: u32,
}


#[account]
pub struct Tournament {
    pub owner: Pubkey,
    pub tournament_list: Vec<TournamentStruct>,
    pub prize_pool: u32,
}