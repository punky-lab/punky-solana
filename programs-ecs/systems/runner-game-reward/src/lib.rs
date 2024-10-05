use bolt_lang::*;
use punky_status::PunkyStatus;

declare_id!("BMGw6MuBpr4E9Hk1SNNH2DbsQ2acA9yhPKWKkjT5y8Hj");

#[system]
pub mod runner_game_reward {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let position = &mut ctx.accounts.position;
        position.x += 1;
        position.y += 1;
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub position: PunkyStatus,
    }

}
