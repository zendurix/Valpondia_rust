use rltk::Rltk;

use crate::ecs::{game_state::RunState, State};

mod input;
mod movement;

pub use input::InputType;

pub fn player_turn(gs: &mut State) {
    input::handle_input(gs);
    movement::move_player(gs);
}

pub fn try_get_player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    input::get_input(gs, ctx);
    if ctx.key.is_some() {
        RunState::PlayerTurn
    } else {
        RunState::AwaitingInput
    }
}
