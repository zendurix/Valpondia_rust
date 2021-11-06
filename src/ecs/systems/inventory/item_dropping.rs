use specs::prelude::*;

use crate::{ecs::components, gamelog::GameLog};

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, components::WantsToDropItem>,
        ReadStorage<'a, components::Name>,
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::InInventory>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            player,
            mut gamelog,
            entities,
            mut wants_drop,
            names,
            mut positions,
            mut inventory,
        ) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let drop_pos = positions.get(entity).unwrap().clone();

            positions
                .insert(to_drop.item, drop_pos)
                .expect("Unable to insert position to drop");
            inventory.remove(to_drop.item);

            if entity == *player {
                gamelog.entries.push(format!(
                    "You drop the {}.",
                    names.get(to_drop.item).unwrap().name
                ));
            }
        }

        wants_drop.clear();
    }
}
