use rltk::Rltk;
use specs::{Join, WorldExt};

use crate::ecs::{Position, Renderable, State};

pub fn draw_entities(gs: &State, ctx: &mut Rltk) {
    let positions = gs.ecs.read_storage::<Position>();
    let renderables = gs.ecs.read_storage::<Renderable>();
    for (position, render) in (&positions, &renderables).join() {
        if position.level == gs.current_level {
            ctx.set(position.x, position.y, render.fg, render.bg, render.ascii);
        }
    }
}
