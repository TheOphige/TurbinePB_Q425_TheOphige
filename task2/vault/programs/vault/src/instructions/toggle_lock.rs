use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::events::ToggleLockEvent;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    // Only the vault authority can toggle the lock
    #[account(mut, has_one = vault_authority)]
    pub vault: Account<'info, Vault>,
    #[account(mut, signer)]
    pub vault_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    // Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
    vault.locked = !vault.locked;

    // Emit a toggle lock event after successful state change
    emit!(ToggleLockEvent {
        vault: vault.key(),
        vault_authority: vault.vault_authority,
        locked: vault.locked,
    });

    Ok(())
}