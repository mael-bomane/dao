use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::errors::ErrorCode;
use crate::state::{Analytics, User, DAO};

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct StakeClaim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"auth"],
        bump = analytics.auth_bump
    )]
    /// CHECK: This is safe, account doesn't exists nor holds data
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"dao", dao.creator.as_ref(), dao.mint.as_ref()],
        bump = dao.dao_bump
    )]
    pub dao: Box<Account<'info, DAO>>,
    #[account(
        mut,
        init_if_needed,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub signer_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        address = dao.mint @ ErrorCode::WrongDAOMint
    )]
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [b"vault", dao.creator.as_ref(), dao.mint.as_ref()],
        token::mint = mint,
        token::authority = auth,
        bump = dao.vault_bump
    )]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"analytics"],
        bump = analytics.state_bump
    )]
    pub analytics: Box<Account<'info, Analytics>>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakeClaim<'info> {
    pub fn stake_claim(&mut self) -> Result<()> {
        // pub creator: Pubkey,
        // pub mint: Pubkey,
        // pub time: Time,
        // pub approved: u64,
        // pub rejected: u64,
        // pub created_at: i64,
        // pub bump: u8,
        // pub name: String,
        // pub polls: Vec<Poll>,
        // pub users: Vec<User>,
        let dao = &mut self.dao;

        let user = dao
            .users
            .clone()
            .into_iter()
            .find(|user| &user.user == &self.user.clone().key());

        match user {
            Some(user) => {
                let mut deposits = user.deposits;

                for i in 0..deposits.len() {
                    deposits[i].deactivating = true;
                    deposits[i].deactivation_start = Some(Clock::get()?.unix_timestamp);
                }

                let index = dao
                    .users
                    .clone()
                    .into_iter()
                    .position(|user| &user.user == &self.user.clone().key())
                    .unwrap();

                let _ = std::mem::replace(
                    &mut dao.users[index],
                    User {
                        user: user.user,
                        voting_power: 0u64,
                        points: user.points,
                        created_at: user.created_at,
                        deposits,
                    },
                );
            }
            None => return err!(ErrorCode::NoDepositsForThisUserInThisDAO),
        }

        Ok(())
    }
}