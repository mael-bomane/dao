use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Analytics, Poll, DAO},
};

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: String, content: String, amount: u64)]
pub struct PollCreate<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        seeds = [b"dao", dao.creator.as_ref(), dao.mint.as_ref()],
        bump = dao.dao_bump 
    )]
    pub dao: Box<Account<'info, DAO>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = creator,
    )]
    pub signer_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        address = dao.mint @ ErrorCode::WrongDAOMint
    )]
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [b"analytics"],
        bump = analytics.state_bump
    )]
    pub analytics: Box<Account<'info, Analytics>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> PollCreate<'info> {
    pub fn poll_create(&mut self, title: String, content: String, amount: u64) -> Result<()> {
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
        if title.len() > MAX_DAO_NAME_LENGTH {
            return err!(ErrorCode::DAONameTooLong);
        } else if title.len() == 0 {
            return err!(ErrorCode::DAONameEmpty);
        }

                if title.len() > MAX_DAO_NAME_LENGTH {
            return err!(ErrorCode::DAONameTooLong);
        } else if title.len() == 0 {
            return err!(ErrorCode::DAONameEmpty);
        }


        let dao = &mut self.dao;
               Ok(())
    }

    pub fn update_analytics(&mut self) -> Result<()> {
        let analytics = &mut self.analytics;
        analytics.daos += 1;
        Ok(())
    }
}
