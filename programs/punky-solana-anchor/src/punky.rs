use anchor_lang::prelude::*;

#[account]
pub struct PunkyAccount {
    pub health: i32,
    pub fitness: i32,
    pub loyalty: i32,
    pub happiness: i32,
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
