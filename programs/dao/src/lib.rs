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

    pub fn stake_new(ctx: Context<StakeNew>, amount: u64) -> Result<()> {
        ctx.accounts.stake_new(amount)
    }

    pub fn stake_deactivate(ctx: Context<StakeDeactivate>) -> Result<()> {
        ctx.accounts.stake_deactivate()
    }

    pub fn stake_claim(ctx: Context<StakeClaim>) -> Result<()> {
        ctx.accounts.stake_claim()
    }

    pub fn poll_new(ctx: Context<PollNew>, title: String, content: String) -> Result<()> {
        ctx.accounts.poll_new(title, content)?;
        ctx.accounts.update_analytics()
    }

    pub fn vote_new(ctx: Context<VoteNew>, poll: u64, choice: Choice) -> Result<()> {
        ctx.accounts.vote_new(poll, choice)?;
        ctx.accounts.update_analytics()
    }

    pub fn poll_execute(ctx: Context<PollExecute>, poll: u64) -> Result<()> {
        ctx.accounts.poll_execute(poll)
    }
}
