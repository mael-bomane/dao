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

    pub fn dao_create(ctx: Context<DAOCreate>, time: Time) -> Result<()> {
        ctx.accounts.dao_create(&ctx.bumps, time)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
