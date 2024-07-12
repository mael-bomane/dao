use anchor_lang::prelude::*;

use crate::constants::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Time {
    TwentyFourHours,
    FourtyEightHours,
    OneWeek
}

impl Time {
    pub fn value(&self) -> i64 {
        match *self {
            Time::TwentyFourHours => ONE_DAY_IN_SECONDS,
            Time::FourtyEightHours => TWO_DAY_IN_SECONDS,
            Time::OneWeek => ONE_WEEK_IN_SECONDS,
        }
    }
}

#[account]
pub struct DAO {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub time: i64,
    pub threshold: u8,
    pub min_poll_tokens: u64,
    pub approved: u64,
    pub rejected: u64,
    pub created_at: i64,
    pub dao_bump: u8,
    pub vault_bump: u8,
    pub name: String,
    pub polls: Vec<Poll>,
    pub users: Vec<User>,
}

impl DAO {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2 // creator, mint 
        + 1 + 8
        + 1 // threshold 51 => 100
        + 8 * 3 // approved, rejected, min_poll_tokens 
        + TIMESTAMP_LENGTH * 2 // time, created_at
        + BUMP_LENGTH // bump
        + VECTOR_LENGTH_PREFIX * 2
        + STRING_LENGTH_PREFIX
        + MAX_DAO_NAME_LENGTH; 
    pub fn total_deposits(&self) -> usize {
        self.users.iter().map(|user| user.deposits.len()).sum()
    }
    pub fn total_polls(&self) -> usize {
        self.polls.len()
    }

    pub fn total_deposit_amount(&self) -> u64 {
        self.users.iter().map(|user| {
            user.deposits.iter().map(|deposit| deposit.amount).sum::<u64>()
        }).sum()
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Choice {
    Approve,
    Reject,
}


#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Poll {
    pub creator: Pubkey,
    pub dao: Pubkey,
    pub created_at: i64,
    pub bump: u8,
    pub executed: bool,
    pub title: String,
    pub content: String,
    pub votes: Vec<Vote>,
}

impl Poll {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2 // creator, dao 
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH 
        + 1 
        + STRING_LENGTH_PREFIX * 2
        + MAX_TITLE_LENGTH
        + MAX_CONTENT_LENGTH
        + VECTOR_LENGTH_PREFIX; // bump
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Vote {
    pub user: Pubkey,
    pub voting_power: u64,
    pub choice: Choice,
    pub created_at: i64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct User {
    pub user: Pubkey,
    pub voting_power: u64,
    pub points: u64,
    pub created_at: i64,
    pub deposits: Vec<Deposit>,
}

impl User {
    pub const LEN: usize = PUBLIC_KEY_LENGTH
        + 8
        + TIMESTAMP_LENGTH
        + VECTOR_LENGTH_PREFIX;

    pub fn total_user_deposit_amount(&self) -> u64 {
        self.deposits.iter().map(|deposit| deposit.amount).sum()
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Deposit {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub deactivating: bool,
    pub deactivation_start: Option<i64>,
    pub created_at: i64,
}

impl Deposit {
    pub const LEN: usize = PUBLIC_KEY_LENGTH * 2 
        + 8 // amount
        + 1 // bool
        + 1 // option
        + TIMESTAMP_LENGTH * 2;
}
