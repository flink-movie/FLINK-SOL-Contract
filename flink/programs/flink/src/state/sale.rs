use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum SaleType {
    Film,
    Extra,
}

#[account]
#[derive(Default)]
pub struct Sale {
    pub id:           u64,
    pub token_mint:   Pubkey,
    pub token_account: Pubkey,       // seller’s ATA holding 1 unit
    pub seller:       Pubkey,
    pub price:        u64,
    pub sale_type:    SaleType,
    pub active:       bool,
    pub bump:         u8,
}
