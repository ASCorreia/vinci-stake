use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Metadata Account is empty")]
    MetadataAccountEmpty,
    #[msg("Invalid Mint Metadata")]
    InvalidMintMetadata,
    #[msg("Invalid Mint")]
    InvalidMint,
    #[msg("Invalid Mint Owner")]
    InvalidMintOwner,
    #[msg("Missing Creators")]
    MissingCreators,
    #[msg("Invalid Stake Pool")]
    InvalidStakePool,
    #[msg("Original Mint Not Claimed")]
    OriginalMintNotClaimed,
    #[msg("Staked Mint Already Claimed")]
    MintAlreadyClaimed,
    #[msg("Unauthorized Signer")]
    UnauthorizedSigner,

}