use anchor_lang::prelude::*;

#[error_code]
pub enum PunkyGameError {
    #[msg("Not enough fitness.")]
    NotEnoughFitness,
}