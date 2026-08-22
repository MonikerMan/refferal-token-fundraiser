use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ReferredContributor {
    pub contributor: Pubkey,
    pub referrer_profile: Pubkey,
    pub contribution_amount: u64,
    pub contributor_reward: u64,
    pub referrer_reward: u64,
    pub claimed: bool,
    pub bump: u8,
}
