use anchor_lang::prelude::*;

declare_id!("ACvLXX4zcamroMhhxusoTEeYKTqtTqNvpMpboCJfyrvS");

#[program]
pub mod punky_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
