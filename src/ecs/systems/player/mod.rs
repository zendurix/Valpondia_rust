use rltk::Rltk;
use specs::{Entity, WorldExt};

use crate::ecs::{
    components,
    game_state::{GameLog, RunState},
    State,
};

pub mod input;
mod movement;

pub use input::InputType;
pub use movement::Dir;

pub fn try_player_turn(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let mut is_sleeping = false;
    {
        let player = *gs.ecs.fetch::<Entity>();
        let mut gamelog = gs.ecs.fetch_mut::<GameLog>();
        let mut sleeping_effects = gs.ecs.write_storage::<components::SleepingEffect>();

        if let Some(sleep) = sleeping_effects.get_mut(player) {
            sleep.duration -= 1;
            if sleep.duration < 1 {
                sleeping_effects.remove(player);
            }
            gamelog
                .entries
                .push("You are sleeping. Skipping one turn.".to_string());
            is_sleeping = true;
        }
    }
    if is_sleeping {
        return RunState::PlayerTurn;
    }

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
