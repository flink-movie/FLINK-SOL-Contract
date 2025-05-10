use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint};
use crate::state::Film;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(args: CreateFilmArgs)]
pub struct CreateFilm<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: will be initialized
    #[account(
        init,
        payer = creator,
        seeds = [FILM_SEED, &args.id.to_le_bytes()],
        bump,
        space = 8 + std::mem::size_of::<Film>() + 4 + args.uri.len()
    )]
    pub film: Account<'info, Film>,

    // Platform vault gets royalty + platform rake
    #[account(
        seeds = [PLATFORM_VAULT_SEED],
        bump,
    )]
    /// CHECK: just PDA
    pub platform_vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateFilmArgs {
    pub id:          u64,
    pub uri:         String,
    pub supply:      u32,
    pub price:       u64,
    pub royalty_bps: u16,
}
