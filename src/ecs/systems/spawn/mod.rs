use crate::{components, ecs::State, levels::level::Level, spawner::spawn_entity};
use itertools::Itertools;
use specs::prelude::*;

pub fn spawn_system(gs: &mut State) {
    let mut spawns_with_pos: Vec<(String, (usize, usize))> = vec![];
    let level_index = gs.ecs.fetch::<Level>().level_index;

    {
        let positions = gs.ecs.read_storage::<components::Position>();
        let mut spawns = gs.ecs.write_storage::<components::Spawn>();
        let level = gs.ecs.fetch::<Level>();

        for (pos, spawn) in (&positions, &spawns).join() {
            for (name, num) in spawn.names_nums.iter() {
                let spawns_pos = level.map.closest_not_blocked_positions(
                    (pos.x, pos.y),
                    *num,
                    &spawns_with_pos
                        .iter()
                        .map(|(_name, pos)| *pos)
                        .collect_vec(),
                );

                for pos in spawns_pos {
                    spawns_with_pos.push((name.clone(), pos));
                }
            }
        }
        spawns.clear();
    }

    for (name, pos) in spawns_with_pos {
        spawn_entity(&mut gs.ecs, &name, pos.0, pos.1, level_index);
    }
}
