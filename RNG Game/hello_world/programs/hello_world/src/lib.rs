use anchor_lang::prelude::*;

declare_id!("7UMRekMzg9rd3PF8mTvGMJ4mneQSHrARz6NYM8rimNxm");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
