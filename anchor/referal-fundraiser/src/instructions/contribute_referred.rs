use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked};
use crate::{state::*, reward::calculate_referral_rewards, error::FundraiserError};

#[derive(Accounts)]
#[instruction(purchase_amount: u64, referral_code: String)]
pub struct ContributeReferred<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(address = fundraiser.mint_to_raise)]
    pub mint_to_raise: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"fundraiser", fundraiser.maker.as_ref()],
        bump = fundraiser.bump
    )]
    pub fundraiser: Account<'info, Fundraiser>,

    #[account(
        seeds = [b"referral-code", referral_code.as_bytes()],
        bump = lookup.bump,
    )]
    pub lookup: Account<'info, ReferralCodeLookup>,

    #[account(
        mut,
        seeds = [b"referrer-profile", lookup.referrer_profile.as_ref()],
        bump = referrer_profile.bump,
        constraint = referrer_profile.key() == lookup.referrer_profile
    )]
    pub referrer_profile: Account<'info, ReferrerProfile>,

    #[account(
        init_if_needed,
        payer = contributor,
        space = 8 + ReferredContributor::INIT_SPACE,
        seeds = [
            b"referred-contributor",
            fundraiser.key().as_ref(),
            contributor.key().as_ref()
        ],
        bump
    )]
    pub referred_contributor: Account<'info, ReferredContributor>,

    #[account(
        mut,
        associated_token::mint = mint_to_raise,
        associated_token::authority = contributor,
        associated_token::token_program = token_program,
    )]
    pub contributor_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_to_raise,
        associated_token::authority = fundraiser,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handle_contribute_referred(
    ctx: Context<ContributeReferred>,
    purchase_amount: u64,
    _referral_code: String,
) -> Result<()> {
    // 1. Calculate Rewards Breakdown
    let rewards = calculate_referral_rewards(purchase_amount)?;

    // 2. Update Referred Contributor Ledger
    let ref_account = &mut ctx.accounts.referred_contributor;
    ref_account.contributor = ctx.accounts.contributor.key();
    ref_account.referrer_profile = ctx.accounts.referrer_profile.key();
    ref_account.contribution_amount = ref_account
        .contribution_amount
        .checked_add(purchase_amount)
        .ok_or(FundraiserError::MathOverflow)?;
    ref_account.contributor_reward = ref_account
        .contributor_reward
        .checked_add(rewards.contributor_reward)
        .ok_or(FundraiserError::MathOverflow)?;
    ref_account.referrer_reward = ref_account
        .referrer_reward
        .checked_add(rewards.referrer_reward)
        .ok_or(FundraiserError::MathOverflow)?;
    ref_account.bump = ctx.bumps.referred_contributor;

    // 3. Update Lifetime Profile Totals
    let profile = &mut ctx.accounts.referrer_profile;
    profile.total_rewards_earned = profile
        .total_rewards_earned
        .checked_add(rewards.referrer_reward)
        .ok_or(FundraiserError::MathOverflow)?;

    // 4. Update Fundraiser Target State
    ctx.accounts.fundraiser.current_amount = ctx.accounts
        .fundraiser
        .current_amount
        .checked_add(purchase_amount)
        .ok_or(FundraiserError::MathOverflow)?;

    // 5. CPI: Escrow contribution tokens into Vault
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.contributor_ata.to_account_info(),
        mint: ctx.accounts.mint_to_raise.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.contributor.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    transfer_checked(cpi_ctx, purchase_amount, ctx.accounts.mint_to_raise.decimals())?;

    Ok(())
}
