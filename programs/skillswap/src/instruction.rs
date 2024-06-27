use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};

use crate::error::ErrorCode;
use crate::state::*;
use crate::event::*;

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
