use std::collections::HashSet;

use rltk::RGB;
use specs::{Builder, Entity, EntityBuilder, World, WorldExt};

use crate::{
    ecs::{components, systems::inventory::insert_item_in_inv},
    rng,
    spawner::spawn_tables::SpawnEntry,
};

use super::spawn_entity;

pub fn spawn_item_into_inventory(
    ecs: &mut World,
    owner: Entity,
    item_name: String,
    x: usize,
    y: usize,
    level: usize,
) {
    if let Some(item) = spawn_entity(ecs, &item_name, x, y, level) {
        let is_item;
        {
            let items = ecs.read_storage::<components::Item>();
            is_item = items.contains(item);
        }
        if is_item {
            insert_item_in_inv(ecs, owner, item);
        } else {
            println!("{} isnt item", item_name);
        }
    } else {
        println!("Unable to create {}", item_name);
    }
}

pub fn spawn_random_monster(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let rand = rng::roll_dice(1, 4);

    match rand {
        1 => spawn_goblin(ecs, x, y, level),
        2 => spawn_orc(ecs, x, y, level),
        3 => spawn_human(ecs, x, y, level),
        4 => spawn_knight(ecs, x, y, level),
        _ => panic!("Wrong random number during monster spawning"),
    }
}

pub fn spawn_goblin(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(ecs, x, y, level, rltk::to_cp437('g'), "Goblin", 10, 4, 1).build()
}

pub fn spawn_orc(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let orc = spawn_monster(ecs, x, y, level, rltk::to_cp437('o'), "Orc", 32, 12, 3)
        .with(components::Inventory::new_empty())
        .build();
    spawn_item_into_inventory(ecs, orc, "Dagger".to_string(), x, y, level);
    orc
}

pub fn spawn_human(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(ecs, x, y, level, rltk::to_cp437('h'), "Human", 20, 15, 2).build()
}

pub fn spawn_knight(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let knight = spawn_monster(ecs, x, y, level, rltk::to_cp437('k'), "Knight", 35, 8, 7)
        .with(components::Inventory::new_empty())
        .build();
    spawn_item_into_inventory(ecs, knight, "Chain armor".to_string(), x, y, level);
    spawn_item_into_inventory(ecs, knight, "Zweihander".to_string(), x, y, level);
    knight
}

pub fn spawn_blip(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(ecs, x, y, level, rltk::to_cp437('b'), "Blip", 8, 3, 2).build()
}

pub fn spawn_blop(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let blop = spawn_monster(ecs, x, y, level, rltk::to_cp437('B'), "Blop", 50, 4, 5)
        .with(components::SpawnsAfterDeath {
            spawns: vec![
                SpawnEntry::new("Blip".to_string(), 3, 5),
                SpawnEntry::new("Blop".to_string(), 1, 1).with_chance(10),
            ],
        })
        .with(components::Inventory::new_empty())
        .build();
    spawn_item_into_inventory(ecs, blop, "Gino rossi boots".to_string(), x, y, level);
    blop
}

#[allow(clippy::too_many_arguments)]
fn spawn_monster<S: ToString>(
    ecs: &mut World,
    x: usize,
    y: usize,
    level: usize,
    glyph: rltk::FontCharType,
    name: S,
    hp: i32,
    atk: i32,
    def: i32,
) -> EntityBuilder {
    ecs.create_entity()
        .with(components::Position { x, y, level })
        .with(components::Renderable {
            ascii: glyph,
            texture: None,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(components::Name {
            name: name.to_string(),
        })
        .with(components::View {
            visible_tiles: HashSet::<rltk::Point>::new(),
            range: 10,
            should_update: true,
        })
        .with(components::AI {})
        .with(components::BlocksTile {})
        .with(components::Hp { max_hp: hp, hp })
        .with(components::CombatBaseStats {
            attack: atk,
            defense: def,
        })
}
