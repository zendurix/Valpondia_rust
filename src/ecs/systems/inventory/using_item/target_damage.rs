use lazy_static::__Deref;
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
        ReadStorage<'a, components::AreaOfEffect>,
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
            aoes,
            wants_to_use,
            mut suffers_damages,
        ) = data;

        for (entity, uses) in (&entities, &wants_to_use).join() {
            let damage = inflicts_damages.get(uses.item);

            if let (Some(dmg), Some(target)) = (damage, uses.target) {
                let mut targets: Vec<Entity> = Vec::new();

                let aoe = aoes.get(uses.item);
                match aoe {
                    None => {
                        // Single target
                        let idx = map.xy_to_index(target.x as usize, target.y as usize);
                        for mob in map.tile_content[idx].iter() {
                            targets.push(*mob);
                        }
                    }
                    Some(aoe) => {
                        let mut area_tiles = rltk::field_of_view(target, aoe.radius, map.deref());
                        area_tiles.retain(|p| {
                            p.x > 0
                                && p.x < map.width_max() as i32
                                && p.y > 0
                                && p.y < map.height_max() as i32
                        });
                        for tile_idx in area_tiles.iter() {
                            let idx = map.xy_to_index(tile_idx.x as usize, tile_idx.y as usize);
                            for mob in map.tile_content[idx].iter() {
                                targets.push(*mob);
                            }
                        }
                    }
                }

                for target_ent in targets {
                    components::SufferDamage::new_damage(
                        &mut suffers_damages,
                        target_ent,
                        dmg.damage,
                    );

                    if entity == *player {
                        let name = names.get(target_ent).unwrap();
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
