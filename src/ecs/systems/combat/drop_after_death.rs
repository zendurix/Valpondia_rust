use crate::components;
use specs::prelude::*;

pub struct DropAfterDeathSystem {}

impl<'a> System<'a> for DropAfterDeathSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, components::Hp>,
        WriteStorage<'a, components::Inventory>,
        WriteStorage<'a, components::InInventory>,
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Equipped>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            entities,
            player,
            hps,
            mut inventories,
            mut items_in_inv,
            mut positions,
            mut equippeds,
        ) = data;

        for (ent, _hp, inv) in (&entities, &hps, &mut inventories)
            .join()
            .filter(|(ent, hp, _)| *ent != *player && hp.hp <= 0)
        {
            let drop_pos = positions.get(ent).unwrap().clone();
            for item in inv.items.iter() {
                positions
                    .insert(*item, drop_pos.clone())
                    .expect("Unable to insert position to drop");
                items_in_inv.remove(*item);

                equippeds.remove(*item);
            }
            inv.items.clear();
        }
    }
}
