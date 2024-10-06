mod punky;

use anchor_lang::prelude::*;
use punky::*;

declare_id!("H1LbKLGSBTKUrWiPTU6QiPzodkQaAWEqvMXjhiXu3izU");

#[program]
pub mod punky_solana_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Program {:?} account initialized", ctx.program_id);
        // max 100
        ctx.accounts.new_punky_account.health = 25;
        ctx.accounts.new_punky_account.fitness = 50;
        ctx.accounts.new_punky_account.loyalty = 25;
        Ok(())
    }

    pub fn get_reward(ctx: Context<GetReward>) -> Result<()> {
        let punky_account = &mut ctx.accounts.punky_account;
        if punky_account.loyalty + 5 > 100 {
            msg!("loyalty exceeded limit");
        } else {
            punky_account.loyalty += 5;
            msg!("got 5 loyalty");
        }
        Ok(())
    }

    pub fn run_one_second(ctx: Context<RunOneSecond>) -> Result<()> {
        let punky_account = &mut ctx.accounts.punky_account;
        
        if punky_account.fitness - 5 < 0 {
            msg!("not enough fitness");
        } else {
            punky_account.fitness -= 5;
            msg!("-5 fitness");
        }
        
        if punky_account.loyalty + 5 > 100 {
            msg!("loyalty exceeded limit");
        } else {
            punky_account.loyalty += 5;
            msg!("got 5 loyalty");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        seeds = [b"punky", signer.key().as_ref()],
        bump,
        payer = signer,
        space = 32 * 3 + 1,
    )]
    pub new_punky_account: Account<'info, PunkyAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
