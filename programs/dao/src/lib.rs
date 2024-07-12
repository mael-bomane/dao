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

    use state::Time;

    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)
    }

    pub fn dao_create(
        ctx: Context<DAOCreate>,
        time: Time,
        threshold: u8,
        name: String,
    ) -> Result<()> {
        ctx.accounts.dao_create(&ctx.bumps, time, threshold, name)?;
        ctx.accounts.update_analytics()
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        ctx.accounts.stake(amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
