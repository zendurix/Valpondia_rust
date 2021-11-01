pub mod ai_random_mov;

use rltk::{console, GameState, Rltk};
use specs::{Join, ReadExpect, ReadStorage, System, WorldExt};

use crate::{
    base::start_to_end_as_dir,
    ecs::{components, State},
};

pub struct AISystem {}

impl<'a> System<'a> for AISystem {
    type SystemData = (
        ReadStorage<'a, components::View>,
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::AI>,
        ReadStorage<'a, components::Name>,
        ReadExpect<'a, rltk::Point>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (views, pos, monster, names, player_pos) = data;

        for (view, _pos, _monster, name) in (&views, &pos, &monster, &names).join() {
            if view.visible_tiles.contains(&player_pos) {
                console::log(format!("{} shouts: ARGHHHHHHHHH", name.name));
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn ai_main(gs: &mut State, _ctx: &mut Rltk) {
    let positions = gs.ecs.read_storage::<components::Position>();
    let mut movables = gs.ecs.write_storage::<components::Movable>();
    let names = gs.ecs.read_storage::<components::Name>();
    let ais = gs.ecs.read_storage::<components::AI>();
    let views = gs.ecs.read_storage::<components::View>();

    let player_pos = gs.ecs.read_resource::<rltk::Point>();
    let map = gs.current_map();

    for (view, pos, _ai, name, mut movable) in
        (&views, &positions, &ais, &names, &mut movables).join()
    {
        if view.visible_tiles.contains(&player_pos) {
            //   console::log(format!("{} shouts: ARGHHHHHHHHH", name.name));

            /// following player
            let path = rltk::a_star_search(
                map.xy_to_index(pos.x, pos.y),
                map.xy_to_index(player_pos.x as usize, player_pos.y as usize),
                map,
            );
            if path.success && path.steps.len() > 1 {
                let x = path.steps[1] % map.width;
                let y = path.steps[1] / map.width;
                let mov_dir = start_to_end_as_dir(pos.x, pos.y, x, y);
                movable.move_dir = Some(mov_dir);
                console::log(format!("{} follows", name.name));
            } else {
                console::log(format!("{} NOT follows", name.name));
            }
        }
    }
}
