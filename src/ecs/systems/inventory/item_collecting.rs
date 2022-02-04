use specs::prelude::*;

use crate::{ecs::{components, game_state::GameLog}};

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
        WriteStorage<'a, components::Inventory>,
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
            mut inventories,
        ) = data;

        for (ent, pickup, inv) in (&entities, &wants_pickup, &mut inventories).join() {
            positions.remove(pickup.item);
            in_inventories
                .insert(pickup.item, components::InInventory { owner: ent })
                .expect("Unable to insert backpack entry");
            inv.items.push(pickup.item);

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

pub fn insert_item_in_inv(ecs: &mut World, owner: Entity, item: Entity) {
    let mut positions = ecs.write_storage::<components::Position>();
    let mut in_inv = ecs.write_storage::<components::InInventory>();

    let mut inventories = ecs.write_storage::<components::Inventory>();

    if let Some(inv) = inventories.get_mut(owner) {
        positions.remove(item);
        in_inv
            .insert(item, components::InInventory { owner })
            .expect("Unable to insert backpack entry");
        inv.items.push(item);
    } else {
        println!("Can't insert item into inv, entity doesn;t have invnetory");
    }
}
