use specs::prelude::*;

use crate::{ecs::{components, game_state::GameLog}};

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
        WriteStorage<'a, components::Inventory>,
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
            mut in_inventories,
            mut inventories,
        ) = data;

        for (entity, to_drop, inv) in (&entities, &wants_drop, &mut inventories).join() {
            let drop_pos = positions.get(entity).unwrap().clone();

            positions
                .insert(to_drop.item, drop_pos)
                .expect("Unable to insert position to drop");
            in_inventories.remove(to_drop.item);

            if let Some((i, _item)) = inv
                .items
                .iter()
                .enumerate()
                .find(|(_i, item)| **item == to_drop.item)
            {
                inv.items.remove(i);
            }

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

pub fn drop_item(ecs: &mut World, owner: Entity, item: Entity) {
    let mut positions = ecs.write_storage::<components::Position>();
    let mut in_inv = ecs.write_storage::<components::InInventory>();

    let mut inventories = ecs.write_storage::<components::Inventory>();

    if let Some(inv) = inventories.get_mut(owner) {
        let drop_pos = positions.get(owner).unwrap().clone();

        positions
            .insert(item, drop_pos)
            .expect("Unable to insert position to drop");
        in_inv.remove(item);

        if let Some((i, _item)) = inv.items.iter().enumerate().find(|(_i, it)| **it == item) {
            inv.items.remove(i);
        }
    } else {
        println!("Can't drop item inv, entity doesn;t have invnetory");
    }
}
