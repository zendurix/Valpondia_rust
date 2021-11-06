use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{
    ecs::components::{self},
    gamelog::GameLog,
};

/// used for heal potions.

pub struct ItemHealSystem {}

impl<'a> System<'a> for ItemHealSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        ReadStorage<'a, components::Name>,
        ReadStorage<'a, components::Heal>,
        WriteStorage<'a, components::WantsToUseItem>,
        WriteStorage<'a, components::Hp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            player,
            mut gamelog,
            entities,
            names,
            heals,
            mut wants_to_use,
            mut hps,
        ) = data;

        for (entity, uses, hp) in (&entities, &wants_to_use, &mut hps).join() {
            let heal = heals.get(uses.item);
            if let Some(h) = heal {
                hp.hp = (hp.hp + h.heal_power).min(hp.max_hp);

                if entity == *player {
                    gamelog.entries.push(format!(
                        "You drink the {}, healing {} hp.",
                        names.get(uses.item).unwrap().name,
                        h.heal_power
                    ));
                }

                entities.delete(uses.item).expect("Item Delete failed");
            }
        }

        wants_to_use.clear();
    }
}
