use crate::{ecs::components, levels::level::Level};

use specs::prelude::*;

pub struct MapIndexingSystem {}
impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Level>,
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut current_level, position, blockers, entities) = data;

        current_level.map.update_blocked_with_blocking_tiles();
        current_level.map.clear_tiles_contents();
        for (ent, position) in (&entities, &position).join() {
            let idx = current_level.map.xy_to_index(position.x, position.y);

            if let Some(_block) = blockers.get(ent) {
                current_level.map.blocked[idx] = true;
            }
            current_level.map.tile_content[idx].push(ent);
        }
    }
}
