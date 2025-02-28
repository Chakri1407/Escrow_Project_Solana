use anchor_lang::prelude::*;

pub fn get_escrow_seeds<'a>(authority: &'a Pubkey) -> [&'a [u8]; 2] {
    [crate::constants::ESCROW_SEED, authority.as_ref()]
}

pub fn get_escrow_token_seeds<'a>(authority: &'a Pubkey, mint: &'a Pubkey) -> [&'a [u8]; 3] {
    [crate::constants::ESCROW_SEED, authority.as_ref(), mint.as_ref()]
}