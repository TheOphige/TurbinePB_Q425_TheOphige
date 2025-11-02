use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.vault_authority.as_ref()],
        bump,
        constraint = !vault.locked @ VaultError::VaultLocked
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &ctx.accounts.vault;
    let user = &ctx.accounts.user;

    // Verify user has enough balance
    require!(
        user.to_account_info().lamports() >= amount,
        VaultError::InsufficientBalance
    );

    // Transfer lamports from user to vault PDA
    invoke(
        &transfer(
            &user.key(),
            &vault.key(),
            amount,
        ),
        &[
            user.to_account_info(),
            vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Emit deposit event
    emit!(DepositEvent {
        vault: vault.key(),
        user: user.key(),
        amount,
    });

    Ok(())
}