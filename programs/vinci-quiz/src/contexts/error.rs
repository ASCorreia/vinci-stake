use crate::*;

#[error_code]
pub enum CustomError {
    #[msg("Player not found")]
    PlayerNotFound,
    #[msg("Insufficient Points For Upgrade")]
    InsufficientPoints,
    #[msg("Insufficient Level for Mega Upgrade")]
    InsufficientLevel,
    #[msg("NFT Already Minted")]
    NftAlreadyMinted,
    #[msg("Invalid Authority")]
    InvalidAuthority,
}