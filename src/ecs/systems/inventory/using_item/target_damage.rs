use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{
    ecs::components::{self},
    gamelog::GameLog,
    maps::Map,
};

pub struct ItemTargetedDamageSystem {}

impl<'a> System<'a> for ItemTargetedDamageSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        ReadStorage<'a, components::Name>,
        ReadStorage<'a, components::InflictsDamage>,
        ReadStorage<'a, components::WantsToUseItem>,
        WriteStorage<'a, components::SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            player,
            map,
            mut gamelog,
            entities,
            names,
            inflicts_damages,
            wants_to_use,
            mut suffers_damages,
        ) = data;

        for (entity, uses) in (&entities, &wants_to_use).join() {
            let damage = inflicts_damages.get(uses.item);

            if let (Some(dmg), Some(target)) = (damage, uses.target) {
                let index = map.xy_to_index(target.x as usize, target.y as usize);
                for target_ent in map.tile_content[index].iter() {
                    components::SufferDamage::new_damage(&mut suffers_damages, *target_ent, dmg.damage);

                    if entity == *player {
                        let name = names.get(*target_ent).unwrap();
                        let item_name = names.get(uses.item).unwrap();
                        gamelog.entries.push(format!(
                            "You use the {} on {} inflicting {} dmg.",
                            item_name.name, name.name, dmg.damage
                        ));
                    }
                }
            }
        }
    }
}
