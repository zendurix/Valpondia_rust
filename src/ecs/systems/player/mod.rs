use rltk::Rltk;
use specs::{Entity, WorldExt};

use crate::ecs::{components, game_state::RunState, State};

pub mod input;
mod movement;

pub use input::InputType;

pub fn try_player_turn(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let input_is_some;
    {
        let player = *gs.ecs.fetch::<Entity>();
        let mut players = gs.ecs.write_storage::<components::Player>();
        let mut player_ipout = players.get_mut(player).unwrap();

        player_ipout.input = input::get_input(ctx);
        input_is_some = player_ipout.input.is_some();
    }

    if input_is_some {
        input::try_handle_input(gs)
    } else {
        RunState::AwaitingInput
    }
}
