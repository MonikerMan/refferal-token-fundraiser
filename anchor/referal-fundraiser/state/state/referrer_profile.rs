use anchor_lang::prelude::*;

pub const MAX_USERNAME_LEN: usize = 32;
pub const MAX_TWITTER_HANDLE_LEN: usize = 15;
pub const REFERRAL_CODE_LEN: usize = 7;

#[account]
#[derive(InitSpace)]
pub struct ReferrerProfile {
    pub authority: Pubkey,
    #[max_len(MAX_USERNAME_LEN)]
    pub username: String,
    #[max_len(MAX_TWITTER_HANDLE_LEN)]
    pub twitter_handle: String,
    #[max_len(REFERRAL_CODE_LEN)]
    pub referral_code: String,
    pub total_rewards_earned: u64,
    pub bump: u8,
}
