use anchor_lang::prelude::*;

declare_id!("BkwuLEBLKo7pBMTjQ3HJSf74x9QFRGWArJawafjTXXuf");

#[program]
pub mod punky_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
