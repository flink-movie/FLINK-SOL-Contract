use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token, Mint};

pub mod constants;
pub mod error;
pub mod utils;
pub mod state;
pub mod contexts;
pub mod instructions;

use constants::*;
use error::FlinkError;
use instructions::*;

declare_id!("FLinK111111111111111111111111111111111111111");

/// Routes each ix to its handler (re-exported from `instructions::…`)
#[program]
pub mod flink {
    use super::*;

    pub fn create_film(ctx: Context<CreateFilm>, args: CreateFilmArgs) -> Result<()> {
        instructions::create_film::handler(ctx, args)
    }
    pub fn create_extra(ctx: Context<CreateExtra>, args: CreateExtraArgs) -> Result<()> {
        instructions::create_extra::handler(ctx, args)
    }
    pub fn buy_primary(ctx: Context<BuyPrimary>, args: BuyPrimaryArgs) -> Result<()> {
        instructions::buy_primary::handler(ctx, args)
    }
    pub fn list_resale(ctx: Context<ListResale>, args: ListResaleArgs) -> Result<()> {
        instructions::list_resale::handler(ctx, args)
    }
    pub fn buy_resale(ctx: Context<BuyResale>) -> Result<()> {
        instructions::buy_resale::handler(ctx)
    }
    pub fn cancel_resale(ctx: Context<CancelResale>) -> Result<()> {
        instructions::cancel_resale::handler(ctx)
    }
}
