use crate::*;

#[derive(Accounts)]
pub struct StartTournament<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, seeds = [b"Tournament_1", user.key().as_ref()], bump, payer = user, space = 4000)]
    pub tournament: Account<'info, Tournament>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddPartcipant<'info> {
    pub user: Signer<'info>,
    pub tournament_list: Account<'info, Tournament>,
    pub new_participant: Account<'info, BaseAccount>,
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

#[account]
pub struct Tournament {
    pub owner: Pubkey,
    pub tournament_list: Vec<Pubkey>,
    pub prize_pool: u32,
}