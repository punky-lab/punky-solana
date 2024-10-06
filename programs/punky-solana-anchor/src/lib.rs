mod config;
mod punky;

use anchor_lang::prelude::*;
use config::*;
use punky::*;

declare_id!("H1LbKLGSBTKUrWiPTU6QiPzodkQaAWEqvMXjhiXu3izU");

#[program]
pub mod punky_solana_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Program {:?} account initialized", ctx.program_id);
        // max 100
        ctx.accounts.new_punky_account.health = INITIAL_HEALTH;
        ctx.accounts.new_punky_account.fitness = INITIAL_FITNESS;
        ctx.accounts.new_punky_account.loyalty = INITIAL_LOYALTY;
        ctx.accounts.new_punky_account.happiness = INITIAL_HAPPINESS;

        // Initialize the fitness update timer and last update time
        ctx.accounts.new_punky_account.fitness_update_timer = FITNESS_UPDATE_INTERVAL;

        // Set to current timestamp
        let current_time = Clock::get().unwrap().unix_timestamp;
        ctx.accounts.new_punky_account.last_fitness_update = current_time;
        Ok(())
    }

    pub fn get_reward(ctx: Context<GetReward>) -> Result<()> {
        let punky_account = &mut ctx.accounts.punky_account;
        if punky_account.loyalty + LOYALTY_ADD_ON_REWARD > MAX_LOYALTY {
            msg!("loyalty exceeded limit");
        } else {
            punky_account.loyalty += LOYALTY_ADD_ON_REWARD;
        }
        Ok(())
    }

    pub fn run_one_second(ctx: Context<RunOneSecond>) -> Result<()> {
        let punky_account = &mut ctx.accounts.punky_account;

        if punky_account.fitness - FITNESS_REDUCE_ON_RUN_ONE_SECOND < 0 {
            msg!("not enough fitness");
        } else {
            punky_account.fitness -= FITNESS_REDUCE_ON_RUN_ONE_SECOND;
        }

        if punky_account.loyalty + LOYALTY_ADD_ON_RUN_ONE_SECOND > MAX_LOYALTY {
            msg!("loyalty exceeded limit");
        } else {
            punky_account.loyalty += LOYALTY_ADD_ON_RUN_ONE_SECOND;
        }
        Ok(())
    }

    pub fn update_fitness(ctx: Context<UpdateFitness>) -> Result<()> {
        let punky_account = &mut ctx.accounts.punky_account;
        let current_time = Clock::get().unwrap().unix_timestamp;

        // Calculate the elapsed time since the last update
        let elapsed_time = current_time - punky_account.last_fitness_update;

        // Determine how many points to add
        let points_to_add = (elapsed_time / FITNESS_UPDATE_INTERVAL) as i32;

        if points_to_add > 0 && punky_account.fitness < MAX_FITNESS {
            // Update the fitness points, ensuring it doesn't exceed 100
            punky_account.fitness = (punky_account.fitness + points_to_add).min(MAX_FITNESS);

            // Update the last fitness update time
            punky_account.last_fitness_update = current_time;
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
        space = 32 * 4 + 64 * 2 + 1,
    )]
    pub new_punky_account: Account<'info, PunkyAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
