pub mod destroy_used_items;
mod heal;
mod sleep;
mod target_damage;

use lazy_static::__Deref;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::{
    ecs::components::{self},
    gamelog::GameLog,
    maps::Map,
};

use self::{heal::use_heal_item, sleep::use_sleep_item, target_damage::use_damage_item};

pub struct UseItemSystem {}

impl<'a> System<'a> for UseItemSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        ReadStorage<'a, components::Name>,
        ReadStorage<'a, components::InflictsDamage>,
        ReadStorage<'a, components::Heal>,
        WriteStorage<'a, components::HealEffect>,
        ReadStorage<'a, components::AreaOfEffect>,
        ReadStorage<'a, components::WantsToUseItem>,
        WriteStorage<'a, components::SufferDamage>,
        ReadStorage<'a, components::Sleeping>,
        WriteStorage<'a, components::SleepingEffect>,
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
            heals,
            mut heals_effects,
            aoes,
            wants_to_use,
            mut suffers_damages,
            sleeps,
            mut sleeps_effects,
        ) = data;

        for (user, uses, user_name) in (&entities, &wants_to_use, &names).join() {
            let item = uses.item;
            let item_name = names.get(item).unwrap();

            let user_name = if user == *player {
                "You".to_string()
            } else {
                user_name.name.clone()
            };

            let target_name = if let Some(t) = uses.target {
                let idx = map.xy_to_index(t.x as usize, t.y as usize);
                // TODO for now just use first one. Maybe use render_order in future
                if !map.tile_content[idx].is_empty() {
                    let e = map.tile_content[idx][0];
                    names.get(e).unwrap().name.clone()
                } else {
                    "nothing".to_string()
                }
            } else {
                "yourself".to_string()
            };

            gamelog.entries.push(format!(
                "{} use {} on {}.",
                user_name, item_name.name, target_name
            ));

            let mut targets: Vec<Entity> = Vec::new();
            if let Some(target) = uses.target {
                let mut area_tiles = vec![];
                match aoes.get(uses.item) {
                    // Single target
                    None => {
                        area_tiles.push(target);
                    }
                    Some(aoe) => {
                        area_tiles.extend(
                            rltk::field_of_view(target, aoe.radius, map.deref())
                                .iter()
                                .filter(|p| {
                                    p.x > 0
                                        && p.x < map.width_max() as i32
                                        && p.y > 0
                                        && p.y < map.height_max() as i32
                                }),
                        );
                    }
                }
                for tile_idx in area_tiles.iter() {
                    let idx = map.xy_to_index(tile_idx.x as usize, tile_idx.y as usize);
                    for ent in map.tile_content[idx].iter() {
                        targets.push(*ent);
                    }
                }
            } else {
                targets.push(user);
            }

            let is_healing = heals.get(item).is_some();
            if is_healing {
                let heal = heals.get(item).unwrap();
                use_heal_item(*player, user, heal, targets.clone(), &mut heals_effects);
            }

            let is_damaging = inflicts_damages.get(item).is_some();
            if is_damaging {
                let dmg = inflicts_damages.get(item).unwrap().clone();
                use_damage_item(*player, user, &dmg, targets.clone(), &mut suffers_damages);
            }

            let is_aplying_sleep = sleeps.get(item).is_some();
            if is_aplying_sleep {
                let sleep = sleeps.get(item).unwrap().clone();
                use_sleep_item(*player, user, &sleep, targets.clone(), &mut sleeps_effects);
            }
        }
    }
}
