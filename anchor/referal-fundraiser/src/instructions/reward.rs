use anchor_lang::prelude::*;

const TOKEN_DECIMALS: u64 = 1_000_000_000;

// Caps represented in raw base units
const MAX_CONTRIBUTOR_REWARD: u64 = 1_000_000 * TOKEN_DECIMALS;
const MAX_REFERRER_REWARD: u64 = 100_000 * TOKEN_DECIMALS; // referrer_reward == contributor_reward \ MAX_CONTRIBUTOR_REWARD

// 3% represented in Basis Points (1 BPS = 0.01%)
const REFERRAL_FEE_BPS: u64 = 300; 
const BPS_DENOMINATOR: u64 = 10_000;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct RewardBreakdown {
    pub contributor_reward: u64,
    pub referrer_reward: u64,
}

pub fn calculate_referral_rewards(purchase_amount: u64) -> Result<RewardBreakdown> {
    // 1] Calculate 3% Contributor Reward using u128 to prevent integer overflow
    let raw_contributor_reward = (purchase_amount as u128)
        .checked_mul(REFERRAL_FEE_BPS as u128)
        .ok_or(ErrorCode::MathOverflow)?
        / (BPS_DENOMINATOR as u128);

    // Clamp the reward at 1,000,000 tokens
    let contributor_reward = (raw_contributor_reward as u64).min(MAX_CONTRIBUTOR_REWARD);

    // 2] Calculate Referrer Reward proportionally based on Contributor Reward
    // Proportion = contributor_reward / MAX_CONTRIBUTOR_REWARD
    // Referrer Reward = MAX_REFERRER_REWARD * (contributor_reward / MAX_CONTRIBUTOR_REWARD)
    let referrer_reward = (MAX_REFERRER_REWARD as u128)
        .checked_mul(contributor_reward as u128)
        .ok_or(ErrorCode::MathOverflow)?
        / (MAX_CONTRIBUTOR_REWARD as u128);

    Ok(RewardBreakdown {
        contributor_reward,
        referrer_reward: referrer_reward as u64,
    })
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow occurred during reward calculation.")]
    MathOverflow,
}
