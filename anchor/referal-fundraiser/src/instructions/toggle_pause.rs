use anchor_lang::prelude::*;
use crate::state::Fundraiser;
use crate::error::FundraiserError;

#[derive(Accounts)]
pub struct TogglePause<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"fundraiser", maker.key().as_ref()],
        bump = fundraiser.bump,
        has_one = maker @ FundraiserError::Unauthorized
    )]
    pub fundraiser: Account<'info, Fundraiser>,
}

pub fn handle_toggle_pause(ctx: Context<TogglePause>) -> Result<()> {
    let fundraiser = &mut ctx.accounts.fundraiser;
    fundraiser.is_paused = !fundraiser.is_paused;
    Ok(())
}
