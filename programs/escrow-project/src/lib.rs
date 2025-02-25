#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

declare_id!("4KABghP5PRfddU5DTWEF74ASg1CkzZ6b5mzURiZZzVxW");

#[program]
pub mod basic_escrow {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
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

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // Check if user has enough funds
        let escrow = &mut ctx.accounts.escrow;
        require!(escrow.amount >= amount, EscrowError::InsufficientFunds);
        require!(escrow.authority == ctx.accounts.authority.key(), EscrowError::Unauthorized);
    
        // Transfer tokens from escrow token account to user's token account
        // IMPORTANT: Fix the PDA signer seeds - they should match the seeds used to create the account
        let authority_key = ctx.accounts.authority.key();
        let mint_key = ctx.accounts.mint.key();
        
        let seeds = &[
            b"escrow".as_ref(),
            authority_key.as_ref(),
            mint_key.as_ref(),
            &[ctx.bumps.escrow_token_account], // Make sure this bump matches the one in your account constraint
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

#[account]
pub struct Escrow {
    pub authority: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum EscrowError {
    #[msg("Insufficient funds in escrow")]
    InsufficientFunds,
    #[msg("Unauthorized to withdraw from this escrow")]
    Unauthorized,
}

}