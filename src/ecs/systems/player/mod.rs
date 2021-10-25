use rltk::Rltk;

use crate::ecs::State;

mod input;

/// true if player finished his turn
pub fn try_player_turn(gs: &mut State, ctx: &mut Rltk) -> bool {
    input::get_input(gs, ctx);

    if ctx.key.is_some() {
        input::handle_input(gs, ctx);
        true
    } else {
        false
    }
}
