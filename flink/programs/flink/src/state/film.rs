use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
#[derive(Default)]
pub struct Film {
    pub id:             u64,
    pub creator:        Pubkey,
    pub uri:            String,  // off-chain JSON
    pub supply:         u32,     // total minted copies
    pub remaining:      u32,     // unsold
    pub price:          u64,     // primary sale price (in smallest units of payout mint)
    pub royalty_bps:    u16,     // 0-10 000
    pub bump:           u8,
}
