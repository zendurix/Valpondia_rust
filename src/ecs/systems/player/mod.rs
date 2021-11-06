use rltk::Rltk;

use crate::ecs::{game_state::RunState, State};

mod input;
mod movement;

pub use input::InputType;

pub fn try_player_turn(gs: &mut State, ctx: &mut Rltk) -> RunState {
    input::get_input(gs, ctx);

    if ctx.key.is_some() {
        if input::try_handle_input(gs) {
            RunState::PlayerTurn
        } else {
            RunState::AwaitingInput
        }
    } else {
        RunState::AwaitingInput
    }
}
