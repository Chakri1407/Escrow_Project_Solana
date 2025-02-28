use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

use crate::errors::EscrowError;
use crate::state::Escrow;

pub fn process(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // Check if user has enough funds
    let escrow = &mut ctx.accounts.escrow;
    require!(escrow.amount >= amount, EscrowError::InsufficientFunds);
    require!(escrow.authority == ctx.accounts.authority.key(), EscrowError::Unauthorized);

    // Transfer tokens from escrow token account to user's token account
    let authority_key = ctx.accounts.authority.key();
    let mint_key = ctx.accounts.mint.key();
    
    let seeds = &[
        b"escrow".as_ref(),
        authority_key.as_ref(),
        mint_key.as_ref(),
        &[ctx.bumps.escrow_token_account],
    ];
    
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.escrow_token_account.to_account_info(),
            },
            &[seeds],
        ),
        amount,
    )?;

    // Update escrow state
    escrow.amount -= amount;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"escrow", authority.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        seeds = [b"escrow", authority.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}