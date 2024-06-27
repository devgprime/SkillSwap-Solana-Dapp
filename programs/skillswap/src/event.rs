use anchor_lang::prelude::*;

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
