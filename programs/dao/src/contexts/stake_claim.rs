use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Analytics, Deposit, User, DAO},
};

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct StakeClaim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"auth", analytics.key().as_ref()],
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
    #[account()]
    /// CHECK : This is safe, we don't read of write from this account
    pub dao_creator: UncheckedAccount<'info>,
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
                let deposits = user.deposits;

                let time = Clock::get()?.unix_timestamp;

                let mut amount_to_claim = 0u64;

                let deposits_to_claim: Vec<Deposit> = deposits
                    .clone()
                    .into_iter()
                    .filter(|deposit| {
                        time > (Some(deposit.deactivation_start).unwrap().unwrap()
                            + ONE_MONTH_IN_SECONDS)
                    })
                    .collect();

                if deposits_to_claim.len() == 0 {
                    return err!(ErrorCode::NoDepositsReadyToClaimForThisUserInThisDAO);
                }

                for i in 0..deposits_to_claim.len() {
                    amount_to_claim += deposits[i].amount;
                }

                let deposits_remaining: Vec<Deposit> = deposits
                    .clone()
                    .into_iter()
                    .filter(|deposit| {
                        time < (Some(deposit.deactivation_start).unwrap().unwrap()
                            + ONE_MONTH_IN_SECONDS)
                    })
                    .collect();

                let remaining_voting_power = deposits_remaining
                    .clone()
                    .into_iter()
                    .map(|deposit| deposit.amount)
                    .sum();

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
                        voting_power: remaining_voting_power,
                        points: user.points,
                        created_at: user.created_at,
                        deposits: deposits_remaining,
                    },
                );

                let accounts = Transfer {
                    from: self.vault.to_account_info(),
                    to: self.signer_ata.to_account_info(),
                    authority: self.auth.to_account_info(),
                };

                let seeds = &[
                    b"auth",
                    self.analytics.to_account_info().key.as_ref(),
                    &[self.analytics.auth_bump],
                ];

                let signer_seeds = &[&seeds[..]];

                let cpi = CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    accounts,
                    signer_seeds,
                );

                transfer(cpi, amount_to_claim)
            }
            None => return err!(ErrorCode::NoDepositsForThisUserInThisDAO),
        }
    }
}
