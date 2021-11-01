use rltk::Rltk;

use crate::ecs::{game_state::RunState, State};

mod input;

pub use input::InputType;

/// true if player finished his turn
pub fn try_player_turn(gs: &mut State, ctx: &mut Rltk) -> RunState {
    input::get_input(gs, ctx);

    if ctx.key.is_some() {
        input::handle_input(gs, ctx);
        RunState::Running
    } else {
        RunState::Paused
    }
}
