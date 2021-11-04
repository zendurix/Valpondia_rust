pub mod ai_random_mov;

use lazy_static::__Deref;
use rltk::{console, GameState, Rltk};
use specs::{Join, ReadExpect, ReadStorage, System, WorldExt};

use crate::{
    base::start_to_end_as_dir,
    ecs::{components, State},
    maps::Map,
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
    let mut positions = gs.ecs.write_storage::<components::Position>();
    let mut movables = gs.ecs.write_storage::<components::Movable>();
    let names = gs.ecs.read_storage::<components::Name>();
    let ais = gs.ecs.read_storage::<components::AI>();
    let mut views = gs.ecs.write_storage::<components::View>();
    let mut blocks = gs.ecs.write_storage::<components::BlocksTile>();
    let entities = gs.ecs.entities();

    let player_pos = gs.ecs.read_resource::<rltk::Point>();
    let mut map = gs.ecs.fetch_mut::<Map>();

    for (ent, mut view, pos, _ai, name, mut movable) in (
        &entities,
        &mut views,
        &mut positions,
        &ais,
        &names,
        &mut movables,
    )
        .join()
    {
        if view.visible_tiles.contains(&player_pos) {
            // following player
            let path = rltk::a_star_search(
                map.xy_to_index(pos.x, pos.y),
                map.xy_to_index(player_pos.x as usize, player_pos.y as usize),
                map.deref(),
            );

            let distance = rltk::DistanceAlg::Pythagoras
                .distance2d(rltk::Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                // Attack goes here
                console::log(&format!("{} shouts insults", name.name));
                return;
            } else if path.success && path.steps.len() > 1 {
                let x = path.steps[1] % map.width;
                let y = path.steps[1] / map.width;
                pos.x = x;
                pos.y = y;

                view.should_update = true;

                if let Some(block) = blocks.get_mut(ent) {
                    map.blocked[block.prev_blocked_tile_index] = false;
                    let curret_index = map.xy_to_index(x, y);
                    map.blocked[curret_index] = true;
                    block.prev_blocked_tile_index = curret_index;
                }
                // let mov_dir = start_to_end_as_dir(pos.x, pos.y, x, y);
                // movable.move_dir = Some(mov_dir);
                console::log(format!("{} follows", name.name));
            } else {
                console::log(format!("{} NOT follows", name.name));
            }
        }
    }
}
