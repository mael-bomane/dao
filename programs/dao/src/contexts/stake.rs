use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::errors::ErrorCode;
use crate::state::{Analytics, Deposit, User, Poll, DAO};

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Stake<'info> {
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
        realloc = DAO::LEN + 
        (if dao.users.iter().any(|i| i.user == user.key())
         {dao.users.len() * User::LEN 
             + (dao.total_deposits() + 1 * Deposit::LEN)
             + (dao.total_polls() * Poll::LEN)}
         else 
         {(dao.users.len() + 1) * User::LEN + (dao.total_deposits() + 1) * Deposit::LEN + (dao.total_polls() * Poll::LEN)}),
        realloc::zero = false,
        realloc::payer = user,
        seeds = [b"dao", dao.creator.as_ref(), dao.mint.as_ref()],
        bump = dao.dao_bump
    )]
    pub dao: Box<Account<'info, DAO>>,
    #[account(
        mut,
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
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, amount: u64) -> Result<()> {
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

        let accounts = Transfer {
            from: self.signer_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer(cpi, amount)?;

        let user = dao
            .users
            .clone()
            .into_iter()
            .find(|user| &user.user == &self.user.clone().key());

        match user {
            Some(user) => {
                let mut deposits = user.deposits;

                let deposit = Deposit {
                    user: self.user.key(),
                    mint: self.mint.key(),
                    amount,
                    deactivating: false,
                    deactivation_start: None,
                    created_at: Clock::get()?.unix_timestamp,
                };

                deposits.push(deposit);

                let index = dao
                    .users
                    .clone()
                    .into_iter()
                    .position(|user| &user.user == &self.user.clone().key())
                    .unwrap();
                let mut voting_power = user.voting_power;
                voting_power += amount;

                let _ = std::mem::replace(
                    &mut dao.users[index],
                    User {
                        user: user.user,
                        voting_power,
                        points: user.points,
                        created_at: user.created_at,
                        deposits,
                    },
                );
            }
            None => {
                let deposit = Deposit {
                    user: self.user.key(),
                    mint: self.mint.key(),
                    amount,
                    deactivating: false,
                    deactivation_start: None,
                    created_at: Clock::get()?.unix_timestamp,
                };

                let user = User {
                    user: self.user.key(),
                    voting_power: amount,
                    points: 0,
                    created_at: Clock::get()?.unix_timestamp,
                    deposits: vec![deposit],
                };

                dao.users.push(user);
            }
        }

        Ok(())
    }
}
