use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Analytics, Deposit, Poll, User, Vote, DAO}, Choice,
};

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(poll: u64, choice: Choice)]
pub struct VoteCreate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        realloc = DAO::LEN + 
        ({
            dao.users.len() * User::LEN 
             + (dao.total_deposits() * Deposit::LEN)
             + (dao.total_polls() * Poll::LEN)
             + (dao.total_votes() + 1 * Vote::LEN)
        }),
        realloc::zero = false,
        realloc::payer = signer,
        seeds = [b"dao", dao.creator.as_ref(), dao.mint.as_ref()],
        bump = dao.dao_bump 
    )]
    pub dao: Box<Account<'info, DAO>>,
    #[account(
        mut,
        seeds = [b"analytics"],
        bump = analytics.state_bump
    )]
    pub analytics: Box<Account<'info, Analytics>>,
    pub system_program: Program<'info, System>,
}

impl<'info> VoteCreate<'info> {
    pub fn vote_create(&mut self, poll: usize, choice: Choice) -> Result<()> {
        // pub creator: Pubkey,
        // pub mint: Pubkey,
        // pub time: Time,
        // pub threshold: u8,
        // pub approved: u64,
        // pub rejected: u64,
        // pub created_at: i64,
        // pub bump: u8,
        // pub name: String,
        // pub polls: Vec<Poll>,
        // pub users: Vec<User>,
        // pub deposits: Vec<Deposit>,
        
        let dao = &mut self.dao;

        let user = &dao
            .users
            .clone()
            .into_iter()
            .find(|user| &user.user == &self.signer.clone().key());

        match user {
            Some(user) => {
                require!(user.total_user_deposit_amount() > 0, ErrorCode::UserHaveNoVotingPowerInThisDAO);
                let poll = &dao.polls[poll];
                require!(Clock::get()?.unix_timestamp < poll.created_at + dao.time, ErrorCode::VotingPeriodExpired);
                let vote = Vote {
                    user: self.signer.key(),
                    voting_power: user.voting_power,
                    choice,
                    created_at: Clock::get()?.unix_timestamp
                };
            },
            None => {
                return err!(ErrorCode::NoDepositsForThisUserInThisDAO)
            }
        }
        
        Ok(())
    }

    pub fn update_analytics(&mut self) -> Result<()> {
        let analytics = &mut self.analytics;
        analytics.daos += 1;
        Ok(())
    }
}
