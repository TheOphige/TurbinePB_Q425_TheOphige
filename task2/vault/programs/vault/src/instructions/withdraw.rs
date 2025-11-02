use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = vault_authority)]
    pub vault: Account<'info, Vault>,
    #[account(mut, signer)]
    pub vault_authority: Signer<'info>,
    #[account(address = anchor_lang::solana_program::system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vault_authority = &ctx.accounts.vault_authority;

    // Verify that the vault is not locked
    require!(!vault.locked, VaultError::VaultLocked);
    // Verify that the vault has enough balance to withdraw
    require!(vault.to_account_info().lamports() >= amount, VaultError::InsufficientBalance);
    
    // Transfer lamports from vault to vault authority
    // subtract from vault
    **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    // add to authority
    **vault_authority.to_account_info().try_borrow_mut_lamports()? += amount;

    // Emit a withdraw event after successful transfer
    emit!(WithdrawEvent {
        vault: vault.key(),
        vault_authority: vault.vault_authority,
        amount,
    });

    Ok(())
}