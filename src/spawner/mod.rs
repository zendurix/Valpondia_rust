use specs::{Entity, World};

use crate::{levels::level::Level, rng, spawner::monsters::spawn_goblin};

use self::{
    items::{
        eq::{
            armors::{
                spawn_chain_armor, spawn_leather_armor, spawn_leather_boots, spawn_plate_armor,
            },
            weapons::{
                spawn_champion_sword, spawn_dagger, spawn_longsword, spawn_shortsword,
                spawn_zweihander,
            },
        },
        potions::{spawn_great_healing_potion, spawn_healing_potion},
        scrolls::{
            spawn_area_sleep_scroll, spawn_fireball_scroll, spawn_magic_missile_scroll,
            spawn_sleep_scroll, spawn_teleport_scroll,
        },
    },
    monsters::{spawn_blip, spawn_blop, spawn_rogue, spawn_knight, spawn_mighty_blop, spawn_orc},
    spawn_tables::SpawnTable,
};

pub mod items;
pub mod monsters;
pub mod player;
pub mod spawn_tables;

pub fn spawn_entity(
    ecs: &mut World,
    name: &str,
    x: usize,
    y: usize,
    level: usize,
) -> Option<Entity> {
    match name {
        "Orc" => Some(spawn_orc(ecs, x, y, level)),
        "Goblin" => Some(spawn_goblin(ecs, x, y, level)),
        "Knight" => Some(spawn_knight(ecs, x, y, level)),
        "Rogue" => Some(spawn_rogue(ecs, x, y, level)),

        "Blip" => Some(spawn_blip(ecs, x, y, level)),
        "Blop" => Some(spawn_blop(ecs, x, y, level)),

        "Health potion" => Some(spawn_healing_potion(ecs, x, y, level)),
        "Great health potion" => Some(spawn_great_healing_potion(ecs, x, y, level)),

        "Magic missile scroll" => Some(spawn_magic_missile_scroll(ecs, x, y, level)),
        "Sleep scroll" => Some(spawn_sleep_scroll(ecs, x, y, level)),
        "Area sleep scroll" => Some(spawn_area_sleep_scroll(ecs, x, y, level)),
        "Fireball scroll" => Some(spawn_fireball_scroll(ecs, x, y, level)),
        "Teleport scroll" => Some(spawn_teleport_scroll(ecs, x, y, level)),

        "Dagger" => Some(spawn_dagger(ecs, x, y, level)),
        "Zweihander" => Some(spawn_zweihander(ecs, x, y, level)),
        "Leather armor" => Some(spawn_leather_armor(ecs, x, y, level)),
        "Leather boots" => Some(spawn_leather_boots(ecs, x, y, level)),
        "Chain armor" => Some(spawn_chain_armor(ecs, x, y, level)),
        "Plate armor" => Some(spawn_plate_armor(ecs, x, y, level)),
        "Long sword" => Some(spawn_longsword(ecs, x, y, level)),
        "Short sword" => Some(spawn_shortsword(ecs, x, y, level)),
        "Mighty blop" => Some(spawn_mighty_blop(ecs, x, y, level)),
        "Champion sword" => Some(spawn_champion_sword(ecs, x, y, level)),

        _ => {
            println!("Cannot spawn {}. Unknown entity", name);
            None
        }
    }
}

pub fn spawn_from_spawn_table(ecs: &mut World, level: &Level, mut spawn_table: SpawnTable) {
    if level.spawn_areas.is_empty() {
        println!("Can't spawn on level without spawn areas!");
        return;
    }

    for spawn_area in level.spawn_areas.iter() {
        let spawn_pack_index = spawn_table.roll_spawn_pack_index(spawn_area.len());
        if let Some(index) = spawn_pack_index {
            let mut spawned_points = vec![];

            for entry in spawn_table.spawn_packs[index].entities.iter() {
                let num = entry.roll_spawn_num();
                let spawn_points = random_spawn_points(num, &spawned_points, spawn_area);

                for (x, y) in spawn_points.into_iter() {
                    if let Some(_ent) =
                        spawn_entity(ecs, &entry.entity_name, x, y, level.level_index)
                    {
                        spawned_points.push((x, y));
                    }
                }
            }
        }
    }
}

fn random_spawn_points(
    num: usize,
    exclude_points: &[(usize, usize)],
    spawn_area: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let mut spawn_points = vec![];
    for _ in 0..num {
        loop {
            let i = rng::range(0, spawn_area.len() as i32 - 1) as usize;
            let point = spawn_area[i];
            if !spawn_points.contains(&point) && !exclude_points.contains(&point) {
                spawn_points.push(point);
                break;
            }
        }
    }
    spawn_points
}
