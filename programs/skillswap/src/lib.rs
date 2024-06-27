use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

mod error;
mod instruction;
mod state;
mod event;

pub use error::ErrorCode;
pub use instruction::*;
pub use state::*;
pub use event::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod skillswap {
    use super::*;

    pub fn create_skill_nft(
        ctx: Context<CreateSkillNft>,
        skill_data: SkillData,
        price: u64,
    ) -> ProgramResult {
        instruction::create_skill_nft(ctx, skill_data, price)
    }

    pub fn list_skill_nft(
        ctx: Context<ListSkillNft>,
        nft_mint: Pubkey,
        price: u64,
    ) -> ProgramResult {
        instruction::list_skill_nft(ctx, nft_mint, price)
    }

    pub fn buy_skill_nft(ctx: Context<BuySkillNft>) -> ProgramResult {
        instruction::buy_skill_nft(ctx)
    }

    pub fn cancel_listing(ctx: Context<CancelListing>) -> ProgramResult {
        instruction::cancel_listing(ctx)
    }

    pub fn update_skill_nft(
        ctx: Context<UpdateSkillNft>,
        new_skill_data: SkillData,
    ) -> ProgramResult {
        instruction::update_skill_nft(ctx, new_skill_data)
    }
}
