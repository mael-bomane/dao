use crate::{
    errors::ErrorCode,
    state::{Analytics, Deposit, Poll, User, Vote, DAO, Status}, Choice,
};

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u64)]
pub struct PollExecute<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"dao", dao.creator.as_ref(), dao.mint.as_ref()],
        bump = dao.dao_bump, 
        constraint = Clock::get()?.unix_timestamp > ( dao.polls[usize::from(index as usize)].created_at + dao.time ) @ ErrorCode::WaitForVotingPeriodToEnd
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

impl<'info> PollExecute<'info> {
    pub fn poll_execute(&mut self, index: u64) -> Result<()> {
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
        
        let dao = &mut self.dao;
        let analytics = &mut self.analytics;
        
        require!(!dao.polls[usize::from(index as usize)].executed, ErrorCode::PollAlreadyExecuted);

        let is_approved = dao.polls[usize::from(index as usize)].is_approved();

        let prev = dao.polls[usize::from(index as usize)].clone();
        
        match is_approved {
            true => {
                dao.approved += 1;
                analytics.approved += 1;
                let _ = std::mem::replace(
                    &mut dao.polls[usize::from(index as usize)],
                    Poll {
                        creator: prev.creator,
                        title: prev.title,
                        content: prev.content,
                        executed: true,
                        status: Status::Approved,
                        created_at: prev.created_at,
                        votes: prev.votes
                    },
                );
            },
            false => {
                dao.rejected += 1;
                analytics.approved += 1;
                let _ = std::mem::replace(
                    &mut dao.polls[usize::from(index as usize)],
                    Poll {
                        creator: prev.creator,
                        title: prev.title,
                        content: prev.content,
                        executed: true,
                        status: Status::Rejected,
                        created_at: prev.created_at,
                        votes: prev.votes
                    },
                );

            }
        }

        dao.reward_points(usize::from(index as usize));
                
        Ok(())
    }
}

