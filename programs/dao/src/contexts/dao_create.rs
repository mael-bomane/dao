use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Analytics, Time, DAO},
};

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(time: Time, threshold: u8, min_poll_tokens: u64, sname: String)]
pub struct DAOCreate<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        seeds = [b"auth", analytics.key().as_ref()],
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
        seeds = [b"vault", creator.key().as_ref(), mint.key().as_ref()],
        token::mint = mint,
        token::authority = auth,
        bump
    )]
    pub vault: Box<Account<'info, TokenAccount>>,
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
    pub fn dao_create(
        &mut self,
        bumps: &DAOCreateBumps,
        time: Time,
        threshold: u8,
        min_poll_tokens: u64,
        name: String,
    ) -> Result<()> {
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
        if name.len() > MAX_DAO_NAME_LENGTH {
            return err!(ErrorCode::DAONameTooLong);
        } else if name.len() == 0 {
            return err!(ErrorCode::DAONameEmpty);
        }
        require!(
            threshold >= 50 && threshold <= 100,
            ErrorCode::ThresholdError
        );

        let dao = &mut self.dao;
        dao.creator = self.creator.key();
        dao.mint = self.mint.key();
        dao.time = time.value();
        dao.threshold = threshold;
        dao.min_poll_tokens = min_poll_tokens;
        dao.approved = 0;
        dao.rejected = 0;
        dao.created_at = Clock::get()?.unix_timestamp;
        dao.dao_bump = bumps.dao;
        dao.vault_bump = bumps.vault;
        dao.name = name;
        dao.polls = Vec::new();
        dao.users = Vec::new();
        Ok(())
    }

    pub fn update_analytics(&mut self) -> Result<()> {
        let analytics = &mut self.analytics;
        analytics.daos += 1;
        Ok(())
    }
}
