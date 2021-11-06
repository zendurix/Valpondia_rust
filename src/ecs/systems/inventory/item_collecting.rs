use specs::prelude::*;

use crate::{ecs::components, gamelog::GameLog};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, components::WantsToPickupItem>,
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::Name>,
        WriteStorage<'a, components::InInventory>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            player,
            entities,
            mut gamelog,
            mut wants_pickup,
            mut positions,
            names,
            mut in_inventories,
        ) = data;

        for (ent, pickup) in (&entities, &wants_pickup).join() {
            positions.remove(pickup.item);
            in_inventories
                .insert(pickup.item, components::InInventory { owner: ent })
                .expect("Unable to insert backpack entry");

            if ent == *player {
                gamelog.entries.push(format!(
                    "You pick up the {}.",
                    names.get(pickup.item).unwrap().name
                ));
            }
        }

        wants_pickup.clear();
    }
}
