use anchor_lang::prelude::*;

const MAX_PLAYER_ID_LEN: usize = 32;
const MAX_GAME_NAME_LEN: usize = 64;
const MAX_ACHIEVEMENTS: usize = 10;
const MAX_ACHIEVEMENT_LEN: usize = 128;
const MAX_SOCIAL_LINK_LEN: usize = 256;

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
