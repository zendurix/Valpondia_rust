pub mod ai_random_mov;

use lazy_static::__Deref;
use rltk::console;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{ecs::components, maps::Map};

pub struct AISystem {}

#[allow(clippy::type_complexity)]
impl<'a> System<'a> for AISystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, components::View>,
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::AI>,
        ReadStorage<'a, components::Name>,
        ReadStorage<'a, components::BlocksTile>,
        ReadExpect<'a, rltk::Point>,
        WriteExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        WriteStorage<'a, components::WantsToMeleeAtack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut views,
            mut positions,
            ais,
            names,
            tiles_blocks,
            player_position,
            mut current_map,
            player,
            mut wants_to_melee,
        ) = data;

        for (entity, mut view, mut pos, _ai, name) in
            (&entities, &mut views, &mut positions, &ais, &names).join()
        {
            if view.visible_tiles.contains(&player_position) {
                // following player
                let path = rltk::a_star_search(
                    current_map.xy_to_index(pos.x, pos.y),
                    current_map.xy_to_index(player_position.x as usize, player_position.y as usize),
                    current_map.deref(),
                );

                let distance = rltk::DistanceAlg::Pythagoras
                    .distance2d(rltk::Point::new(pos.x, pos.y), *player_position);
                if distance < 1.5 {
                    wants_to_melee
                        .insert(entity, components::WantsToMeleeAtack { target: *player })
                        .expect("Unable to insert attack on player!");
                } else if path.success && path.steps.len() > 1 {
                    let x = path.steps[1] % current_map.width;
                    let y = path.steps[1] / current_map.width;

                    if let Some(_block) = tiles_blocks.get(entity) {
                        let prev_index = current_map.xy_to_index(pos.x, pos.y);
                        current_map.blocked[prev_index] = false;
                        let curret_index = current_map.xy_to_index(x, y);
                        current_map.blocked[curret_index] = true;
                    }
                    pos.x = x;
                    pos.y = y;

                    view.should_update = true;
                } else {
                    ()
                }
            } else {
                ()
            }
        }
    }
}
