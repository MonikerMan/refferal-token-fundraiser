use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ReferralCodeLookup {
    pub referrer_profile: Pubkey,
    pub bump: u8,
}
