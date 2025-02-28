use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Insufficient funds in escrow")]
    InsufficientFunds,
    #[msg("Unauthorized to withdraw from this escrow")]
    Unauthorized,
}