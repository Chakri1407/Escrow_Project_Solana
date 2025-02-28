use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

use crate::state::Escrow;

pub fn process(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Transfer tokens from user's token account to escrow token account
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.escrow_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update escrow state
    let escrow = &mut ctx.accounts.escrow;
    escrow.authority = ctx.accounts.authority.key();
    escrow.amount += amount;

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 32 + 8, // discriminator + pubkey + amount
        seeds = [b"escrow", authority.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init_if_needed,
        payer = authority,
        token::mint = mint,
        token::authority = escrow_token_account,
        seeds = [b"escrow", authority.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}