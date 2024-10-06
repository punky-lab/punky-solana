use anchor_lang::prelude::*;

#[account]
pub struct PunkyAccount {
    pub health: i32,
    pub fitness: i32,
    pub loyalty: i32,
    pub happiness: i32,
    // tracking the last update time (timestamp)
    pub last_fitness_update: i64,
    // time remaining for the next update
    pub fitness_update_timer: i64,
}

#[derive(Accounts)]
pub struct GetReward<'info> {
    #[account(mut)]
    pub punky_account: Account<'info, PunkyAccount>,
}

#[derive(Accounts)]
pub struct RunOneSecond<'info> {
    #[account(mut)]
    pub punky_account: Account<'info, PunkyAccount>,
}

#[derive(Accounts)]
pub struct UpdateFitness<'info> {
    #[account(mut)]
    pub punky_account: Account<'info, PunkyAccount>,
}
