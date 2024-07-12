use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::state::{Analytics, Time, DAO};

use std::collections::BTreeMap;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(time: Time)]
pub struct DAOCreate<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        seeds = [b"auth"],
        bump = analytics.auth_bump
    )]
    /// CHECK: This is safe, account doesn't exists nor holds data
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        space = DAO::LEN,
        payer = creator,
        seeds = [b"dao", creator.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub dao: Box<Account<'info, DAO>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = creator,
    )]
    pub signer_ata: Box<Account<'info, TokenAccount>>,
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = creator,
        seeds = [b"vp_vault", creator.key().as_ref(), mint.key().as_ref()],
        token::mint = mint,
        token::authority = auth,
        bump
    )]
    pub vp_vault: Box<Account<'info, TokenAccount>>,
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

impl<'info> DAOCreate<'info> {
    pub fn dao_create(&mut self, bumps: &BTreeMap<String, u8>, time: Time) -> Result<()> {
        // pub creator: Pubkey,
        // pub mint: Pubkey,
        // pub time: Time,
        // pub approved: u64,
        // pub rejected: u64,
        // pub created_at: i64,
        // pub bump: u8,
        // pub polls: Vec<Poll>,
        // pub users: Vec<User>,
        // pub deposits: Vec<Deposit>,
        let dao = &mut self.dao;
        dao.creator = self.creator.key();
        dao.mint = self.mint.key();
        dao.time = time.value();
        dao.approved = 0;
        dao.rejected = 0;
        dao.created_at = Clock::get()?.unix_timestamp;
        dao.bump = *bumps.get("dao").unwrap();
        dao.polls = Vec::new();
        dao.users = Vec::new();
        dao.deposits = Vec::new();
        Ok(())
    }
}
