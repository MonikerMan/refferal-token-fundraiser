use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(referral_code: String)]
pub struct RegisterReferrer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + ReferrerProfile::INIT_SPACE,
        seeds = [b"referrer-profile", authority.key().as_ref()],
        bump
    )]
    pub referrer_profile: Account<'info, ReferrerProfile>,

    #[account(
        init,
        payer = authority,
        space = 8 + ReferralCodeLookup::INIT_SPACE,
        seeds = [b"referral-code", referral_code.as_bytes()],
        bump
    )]
    pub lookup: Account<'info, ReferralCodeLookup>,

    pub system_program: Program<'info, System>,
}

pub fn handle_register_referrer(
    ctx: Context<RegisterReferrer>,
    referral_code: String,
) -> Result<()> {
    let profile = &mut ctx.accounts.referrer_profile;
    profile.authority = ctx.accounts.authority.key();
    profile.total_rewards_earned = 0;
    profile.bump = ctx.bumps.referrer_profile;

    let lookup = &mut ctx.accounts.lookup;
    lookup.referrer_profile = profile.key();
    lookup.code = referral_code;
    lookup.bump = ctx.bumps.lookup;

    Ok(())
}
