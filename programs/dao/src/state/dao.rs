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
    pub approved: u64,
    pub rejected: u64,
    pub created_at: i64,
    pub bump: u8,
    pub polls: Vec<Poll>,
    pub users: Vec<User>,
    pub deposits: Vec<Deposit>,
}

impl DAO {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2 // creator, mint 
        + 1 + 8
        + 8 * 2 // approved, rejected 
        + TIMESTAMP_LENGTH * 2 // time, created_at
        + BUMP_LENGTH // bump
        + VECTOR_LENGTH_PREFIX * 3; 
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
    pub votes: Vec<Vote>,
}

impl Poll {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2 // creator, dao 
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH 
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
    pub points: u64,
    pub created_at: i64,
    pub deposits: Vec<Deposit>,
}

impl User {
    pub const LEN: usize = PUBLIC_KEY_LENGTH
        + 8
        + TIMESTAMP_LENGTH
        + VECTOR_LENGTH_PREFIX;
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Deposit {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub created_at: i64,
}

impl Deposit {
    pub const LEN: usize = PUBLIC_KEY_LENGTH * 2 
        + 8
        + TIMESTAMP_LENGTH;
}
