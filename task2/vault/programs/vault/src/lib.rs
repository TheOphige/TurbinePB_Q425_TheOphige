use anchor_lang::prelude::*;
mod instructions;
mod state;
mod errors;
mod events;

use instructions::*;

declare_id!("734juxRyhGmtjUXAXeLYLUG8ctGWULvyhnyhHb2WZhDG");

#[program]
pub mod vault {
    use super::*;

    pub fn init_vault(ctx: Context<InitializeVault>, locked: bool) -> Result<()> {
      _init_vault(ctx, locked)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
      _deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
      _withdraw(ctx, amount)
    }

    pub fn toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
      _toggle_lock(ctx)
    }
}