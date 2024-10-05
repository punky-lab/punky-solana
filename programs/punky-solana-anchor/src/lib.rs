use anchor_lang::prelude::*;

declare_id!("H1LbKLGSBTKUrWiPTU6QiPzodkQaAWEqvMXjhiXu3izU");

#[program]
pub mod punky_solana_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
