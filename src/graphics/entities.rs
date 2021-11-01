use rltk::Rltk;
use specs::{Join, WorldExt};

use crate::ecs::{components, State};

pub fn draw_entities(gs: &State, ctx: &mut Rltk) {
    let positions = gs.ecs.read_storage::<components::Position>();
    let renderables = gs.ecs.read_storage::<components::Renderable>();

    let views = gs.ecs.read_storage::<components::View>();
    let players = gs.ecs.read_storage::<components::Player>();
    let (view, _player) = (&views, &players).join().next().unwrap();

    for (pos, render) in (&positions, &renderables).join() {
        if pos.level == gs.current_level
            && view.visible_tiles.contains(&rltk::Point::new(pos.x, pos.y))
        {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.ascii);
        }
    }
}
