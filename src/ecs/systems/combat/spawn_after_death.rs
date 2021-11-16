use crate::components;
use specs::prelude::*;

pub struct SpawnsAfterDeathSystem {}

impl<'a> System<'a> for SpawnsAfterDeathSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, components::Hp>,
        ReadStorage<'a, components::SpawnsAfterDeath>,
        WriteStorage<'a, components::Spawn>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            entities,
            player,
            hps,
            spawns_at_death,
            mut spawns
        ) = data;

        for (ent, _hp, spawn) in (&entities, &hps, &spawns_at_death)
            .join()
            .filter(|(ent, hp, _)| *ent != *player && hp.hp <= 0)
        {
            let mut spawn_event = components::Spawn { names_nums: vec![] };
            for spawn_entry in spawn.spawns.iter() {
                let spawns_num = spawn_entry.roll_spawn_num();
                spawn_event
                    .names_nums
                    .push((spawn_entry.entity_name.clone(), spawns_num));
            }

            spawns
                .insert(ent, spawn_event)
                .expect("Cannot insert Spawn event for SpawnsAfterDeath");
        }
    }
}
