use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};
use crate::constants::*;
use crate::error::FlinkError;

/// Returns `(creator_fee, platform_fee, seller_receives)`
pub fn calculate_fees(
    price: u64,
    royalty_bps: u16,
    platform_bps: u16,
) -> Result<(u64, u64, u64)> {
    let creator_fee   = price
        .checked_mul(royalty_bps as u64).ok_or(FlinkError::MathOverflow)?
        / BPS_DENOMINATOR as u64;
    let platform_fee  = price
        .checked_mul(platform_bps as u64).ok_or(FlinkError::MathOverflow)?
        / BPS_DENOMINATOR as u64;
    let seller_receives = price
        .checked_sub(creator_fee).ok_or(FlinkError::MathOverflow)?
        .checked_sub(platform_fee).ok_or(FlinkError::MathOverflow)?;
    Ok((creator_fee, platform_fee, seller_receives))
}

/// Generic SPL transfer helper (authority already signed)
pub fn spl_transfer<'info>(
    from: &Account<'info, TokenAccount>,
    to:   &Account<'info, TokenAccount>,
    authority: &Signer<'info>,
    amount: u64,
    token_program: &Program<'info, Token>,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: from.to_account_info(),
        to:   to.to_account_info(),
        authority: authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(token_program.to_account_info(), cpi_accounts),
        amount,
    )
}
