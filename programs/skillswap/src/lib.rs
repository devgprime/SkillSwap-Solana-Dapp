use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::mem::size_of;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); // Replace with working program ID

const MAX_PLAYER_ID_LEN: usize = 32; 
const MAX_GAME_NAME_LEN: usize = 64; 
const MAX_ACHIEVEMENTS: usize = 10;
const MAX_ACHIEVEMENT_LEN: usize = 128;
const MAX_SOCIAL_LINK_LEN: usize = 256;

// Error codes
#[error]
mod skillswap {
    use super::*;
    
pub enum ErrorCode {
    #[msg("Invalid seller.")]
    InvalidSeller,
    #[msg("Insufficient funds for sale.")]
    InsufficientFunds,
    #[msg("NFT not listed for sale.")]
    NftNotListed,
    #[msg("Invalid NFT owner.")]
    InvalidOwner,
}

#[program]
pub mod skillswap {
    use super::*;

    pub fn create_skill_nft(
        ctx: Context<CreateSkillNft>,
        skill_data: SkillData,
        price: u64,
    ) -> ProgramResult {
        let skill_nft = &mut ctx.accounts.skill_nft;
        skill_nft.authority = *ctx.accounts.user.key;
        skill_nft.skill_data = skill_data;
        skill_nft.price = price;
        skill_nft.is_listed = false;

        // Mint the NFT
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        token::mint_to(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
            1,
        )?;

        Ok(())
    }

    pub fn list_skill_nft(
        ctx: Context<ListSkillNft>,
        nft_mint: Pubkey,
        price: u64,
    ) -> ProgramResult {
        let skill_nft = &mut ctx.accounts.skill_nft;

        // Check if the user is the NFT owner
        if skill_nft.authority != *ctx.accounts.user.key {
            return Err(ErrorCode::InvalidOwner.into());
        }

        skill_nft.price = price;
        skill_nft.is_listed = true;
        
        emit!(NftListed { nft_mint, price });

        Ok(())
    }

    pub fn buy_skill_nft(ctx: Context<BuySkillNft>) -> ProgramResult {
        let skill_nft = &mut ctx.accounts.skill_nft;
        let seller = &ctx.accounts.seller;
        let buyer = &ctx.accounts.buyer;

        // Check if NFT is listed
        if !skill_nft.is_listed {
            return Err(ErrorCode::NftNotListed.into());
        }

        // Check if buyer has enough funds
        if ctx.accounts.buyer_token_account.amount < skill_nft.price {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Transfer funds from buyer to seller
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.buyer_token_account.to_account_info(),
                    to: ctx.accounts.seller_token_account.to_account_info(),
                    authority: buyer.to_account_info(),
                },
            ),
            skill_nft.price,
        )?;

        // Transfer NFT from seller to buyer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.seller_nft_account.to_account_info(),
                    to: ctx.accounts.buyer_nft_account.to_account_info(),
                    authority: seller.to_account_info(),
                },
                &[&[
                    b"skill_nft",
                    seller.key.as_ref(),
                    &[ctx.bumps.seller_nft_account],
                ]],
            ),
            1,
        )?;

        skill_nft.is_listed = false;

        emit!(NftSold { nft_mint: ctx.accounts.skill_nft.key(), price: skill_nft.price });

        Ok(())
    }

    pub fn cancel_listing(ctx: Context<CancelListing>) -> ProgramResult {
        let skill_nft = &mut ctx.accounts.skill_nft;

        if skill_nft.authority != *ctx.accounts.user.key {
            return Err(ErrorCode::InvalidOwner.into());
        }

        if !skill_nft.is_listed {
            return Err(ErrorCode::NftNotListed.into());
        }

        skill_nft.is_listed = false;
        
        emit!(ListingCanceled { nft_mint: ctx.accounts.skill_nft.key() });

        Ok(())
    }

    pub fn update_skill_nft(
        ctx: Context<UpdateSkillNft>,
        new_skill_data: SkillData,
    ) -> ProgramResult {
        let skill_nft = &mut ctx.accounts.skill_nft;

        if skill_nft.authority != *ctx.accounts.user.key {
            return Err(ErrorCode::InvalidOwner.into());
        }

        skill_nft.skill_data = new_skill_data;

        Ok(())
    }


    // Accounts structs
    #[derive(Accounts)]
    pub struct CreateSkillNft<'info> {
        #[account(init, payer = user, space = 8 + SkillNft::LEN)]
        pub skill_nft: Account<'info, SkillNft>,
        pub user: Signer<'info>,
        pub token_program: AccountInfo<'info>,
        pub mint: AccountInfo<'info>,
        pub token_account: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct ListSkillNft<'info> {
        #[account(mut)]
        pub skill_nft: Account<'info, SkillNft>,
        pub user: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct BuySkillNft<'info> {
        #[account(mut)]
        pub skill_nft: Account<'info, SkillNft>,
        #[account(mut)]
        pub seller_nft_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub buyer_nft_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub seller_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub buyer_token_account: Account<'info, TokenAccount>,
        pub seller: Signer<'info>,
        pub buyer: Signer<'info>,
        pub token_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct CancelListing<'info> {
        #[account(mut)]
        pub skill_nft: Account<'info, SkillNft>,
        pub user: Signer<'info>,
    }

    // Data types
    #[account]
    pub struct SkillNft {
        pub authority: Pubkey,
        pub skill_data: SkillData,
        pub price: u64,
        pub is_listed: bool,
    }

    #[account]
    pub struct SkillData {
        pub player_id: String,
        pub game_name: String,
        pub rank: u32,
        pub playstyle: String,
        pub achievements: Vec<String>,
        pub social_links: Option<SocialLinks>,
        pub created_at: i64,
        pub updated_at: i64,
    }

    pub struct SocialLinks {
        pub twitter: String,
        pub twitch: String,
    }

    // Events
    #[event]
    pub struct NftListed {
        pub nft_mint: Pubkey,
        pub price: u64,
    }

    #[event]
    pub struct NftSold {
        pub nft_mint: Pubkey,
        pub price: u64,
    }

    #[event]
    pub struct ListingCanceled {
        pub nft_mint: Pubkey,
    }

}
