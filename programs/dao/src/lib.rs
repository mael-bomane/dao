use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;
use state::*;

declare_id!("J8adapkvtdxjhnMLPkiX5sLD2uoALJhkSaXSQ47fR6sJ");

#[program]
pub mod dao {

    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)
    }

    pub fn dao_create(
        ctx: Context<DAOCreate>,
        time: Time,
        threshold: u8,
        min_poll_tokens: u64,
        name: String,
    ) -> Result<()> {
        ctx.accounts
            .dao_create(&ctx.bumps, time, threshold, min_poll_tokens, name)?;
        ctx.accounts.update_analytics()
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        ctx.accounts.stake(amount)
    }

    pub fn poll_create(ctx: Context<PollCreate>, title: String, content: String) -> Result<()> {
        ctx.accounts.poll_create(title, content)?;
        ctx.accounts.update_analytics()
    }

    pub fn vote_create(ctx: Context<VoteCreate>, poll: u64, choice: Choice) -> Result<()> {
        ctx.accounts.vote_create(poll, choice)?;
        ctx.accounts.update_analytics()
    }

    pub fn poll_execute(ctx: Context<PollExecute>, poll: u64) -> Result<()> {
        ctx.accounts.poll_execute(poll)
    }
}
