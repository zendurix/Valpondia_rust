use crate::{
    ecs::{components, State},
    levels::level_manager::LevelManager,
    maps::Map,
    rng,
};

use rltk::Rltk;
use specs::prelude::*;

use crate::base::Dir;

pub struct MapIndexingSystem {}
impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.update_blocked_with_blocking_tiles();
        map.clear_tiles_contents();
        for (ent, position) in (&entities, &position).join() {
            let idx = map.xy_to_index(position.x, position.y);

            if let Some(_block) = blockers.get(ent) {
                map.blocked[idx] = true;
            }

            map.tile_content[idx].push(ent);
        }
    }
}
