use anchor_lang::prelude::*;

#[error_code]
pub enum FlinkError {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Royalty BPS must be <= 10 000")]
    InvalidRoyaltyBps,
    #[msg("Sale already fulfilled or cancelled")]
    SaleInactive,
    #[msg("Only seller can cancel")]
    UnauthorizedCancel,
    #[msg("Insufficient supply")]
    SupplyExhausted,
    #[msg("Primary price mismatch")]
    BadPrimaryPrice,
}
