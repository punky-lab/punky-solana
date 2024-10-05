use bolt_lang::*;
use punky_status::PunkyStatus;

declare_id!("8FNce2R1v5WxUzsCWKxrFHnAsQ8xhL8rn6wi9wgTcZ6y");

#[system]
pub mod runner_game_time {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let position = &mut ctx.accounts.position;
        position.loyalty += 1;
        position.fitness -= 1;
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub position: PunkyStatus,
    }

}
