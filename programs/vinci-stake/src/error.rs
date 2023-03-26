use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Metadata Account is empty")]
    MetadataAccountEmpty,
    #[msg("Invalid Mint Metadata")]
    InvalidMintMetadata,
}