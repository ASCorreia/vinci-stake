use crate::*;

#[error_code]
pub enum CustomError {
    #[msg("Failed to Realloc")]
    FailedToRealloc,
    #[msg("Not Enough Liquidity")]
    NotEnoughLiquidity,
    #[msg("Liquidity Pool not found for token")]
    NoPoolFound,
    #[msg("Trying to swap same assets")]
    TryingToSwapSameAssets,
    #[msg("Invalid swap amount")]
    InvalidAmount,
}