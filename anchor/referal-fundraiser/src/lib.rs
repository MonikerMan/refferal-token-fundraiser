use anchor_lang::prelude::*;

declare_id!("Eoiuq1dXvHxh6dLx3wh9gj8kSAUpga11krTrbfF5XYsC");

mod constants;
mod error;
mod instructions;
mod reward; // Pure business/math logic
mod state;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use reward::*;
pub use state::*;

#[program]
pub mod fundraiser {
    use super::*;

    // --- Fundraiser Instructions ---
    pub fn initialize_fundraiser(
        ctx: Context<InitializeFundraiserAccountConstraints>,
        amount: u64,
        duration: u16,
    ) -> Result<()> {
        handle_initialize_fundraiser(&mut ctx.accounts, amount, duration, &ctx.bumps)?;
        Ok(())
    }

    pub fn contribute(
        ctx: Context<ContributeAccountConstraints>,
        amount: u64,
    ) -> Result<()> {
        handle_contribute(&mut ctx.accounts, amount, &ctx.bumps)?;
        Ok(())
    }

    pub fn check_contributions(
        ctx: Context<CheckContributionsAccountConstraints>,
    ) -> Result<()> {
        handle_check_contributions(&mut ctx.accounts)?;
        Ok(())
    }

    pub fn refund(ctx: Context<RefundAccountConstraints>) -> Result<()> {
        handle_refund(&mut ctx.accounts)?;
        Ok(())
    }

    pub fn close_fundraiser(
        ctx: Context<CloseFundraiserAccountConstraints>,
    ) -> Result<()> {
        handle_close_fundraiser(&mut ctx.accounts)?;
        Ok(())
    }

    // --- Referral Instructions ---
    pub fn register_referrer(
        ctx: Context<RegisterReferrer>,
        username: String,
        twitter_handle: String,
        referral_code: String,
    ) -> Result<()> {
        handle_register_referrer(ctx, username, twitter_handle, referral_code)?;
        Ok(())
    }

    pub fn contribute_referred(
        ctx: Context<ContributeReferred>,
        purchase_amount: u64,
        referral_code: String,
    ) -> Result<()> {
        handle_contribute_referred(ctx, purchase_amount, referral_code)?;
        Ok(())
    }
}
