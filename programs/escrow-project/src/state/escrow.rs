use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub authority: Pubkey,
    pub amount: u64,
}