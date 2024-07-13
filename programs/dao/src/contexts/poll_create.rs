use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Analytics, Poll, User, Deposit, Vote, DAO, Status},
};

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: String, content: String)]
pub struct PollCreate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        realloc = DAO::LEN + 
        ({
            dao.users.len() * User::LEN 
             + (dao.total_deposits() * Deposit::LEN)
             + (dao.total_polls() + 1 * Poll::LEN)
             + (dao.total_votes() * Vote::LEN)
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

impl<'info> PollCreate<'info> {
    pub fn poll_create(&mut self, title: String, content: String) -> Result<()> {
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
        if title.len() > MAX_TITLE_LENGTH {
            return err!(ErrorCode::PollTitleEmpty);
        } else if title.len() == 0 {
            return err!(ErrorCode::PollTitleTooLong);
        }

        if content.len() > MAX_CONTENT_LENGTH {
            return err!(ErrorCode::PollContentEmpty);
        } else if title.len() == 0 {
            return err!(ErrorCode::PollContentTooLong);
        }

        let dao = &mut self.dao;

        let user = &dao
            .users
            .clone()
            .into_iter()
            .find(|user| &user.user == &self.signer.clone().key());

        match user {
            Some(user) => {
                require!(user.total_user_deposit_amount() >= dao.min_poll_tokens, ErrorCode::NotEnoughDepositsToStartPoll);
                let poll = Poll {
                    creator: self.signer.key(),
                    created_at: Clock::get()?.unix_timestamp,
                    executed: false,
                    status: Status::Voting,
                    title,
                    content,
                    votes: Vec::new()
                };
                dao.polls.push(poll);
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
