use anchor_lang::prelude::*;

#[error]
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
