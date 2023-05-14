use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Insufficient Balance - SPL")]
    InsufficientBalanceSpl,
    #[msg("Insufficient Balance - SOL")]
    InsufficientBalanceSol,
    #[msg("Wrong Signer")]
    WrongSigner,
}