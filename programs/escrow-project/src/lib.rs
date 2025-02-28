use anchor_lang::prelude::*;

mod constants;
mod errors;
mod processor;
mod state;
mod utils;

use processor::*;

declare_id!("4KABghP5PRfddU5DTWEF74ASg1CkzZ6b5mzURiZZzVxW");

#[program]
pub mod basic_escrow {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        processor::deposit::process(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        processor::withdraw::process(ctx, amount)
    }
}