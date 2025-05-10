use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ExtraMaterial {
    pub id:          u64,
    pub film:        Pubkey,   // parent film PDA
    pub uri:         String,
    pub supply:      u32,
    pub remaining:   u32,
    pub price:       u64,
    pub bump:        u8,
}
