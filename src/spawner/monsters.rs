use std::collections::HashSet;

use rltk::RGB;
use specs::{Builder, Entity, EntityBuilder, World, WorldExt};

use crate::{
    ecs::{
        components,
        systems::inventory::{insert_item_in_eq, insert_item_in_inv},
    },
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
) -> Option<Entity> {
    if let Some(item) = spawn_entity(ecs, &item_name, x, y, level) {
        let is_item;
        {
            let items = ecs.read_storage::<components::Item>();
            is_item = items.contains(item);
        }
        if is_item {
            insert_item_in_inv(ecs, owner, item);
            Some(item)
        } else {
            println!("{} isnt item", item_name);
            None
        }
    } else {
        println!("Unable to create {}", item_name);
        None
    }
}

pub fn spawn_item_in_eq(
    ecs: &mut World,
    owner: Entity,
    item_name: String,
    x: usize,
    y: usize,
    level: usize,
) {
    if let Some(item) = spawn_item_into_inventory(ecs, owner, item_name, x, y, level) {
        insert_item_in_eq(ecs, owner, item);
    }
}

pub fn spawn_random_monster(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let rand = rng::roll_dice(1, 4);

    match rand {
        1 => spawn_goblin(ecs, x, y, level),
        2 => spawn_orc(ecs, x, y, level),
        3 => spawn_rogue(ecs, x, y, level),
        4 => spawn_knight(ecs, x, y, level),
        _ => panic!("Wrong random number during monster spawning"),
    }
}

pub fn spawn_goblin(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('g'),
        Some(3),
        "Goblin",
        10,
        4,
        0,
    )
    .build()
}

pub fn spawn_orc(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let orc = spawn_monster(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('o'),
        Some(4),
        "Orc",
        32,
        12,
        2,
    )
    .with(components::Inventory::new_empty())
    .with(components::BodyParts::default_humanoid())
    .build();
    spawn_item_in_eq(ecs, orc, "Dagger".to_string(), x, y, level);
    orc
}

pub fn spawn_rogue(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('h'),
        Some(6),
        "Rogue",
        15,
        26,
        2,
    )
    .build()
}

pub fn spawn_knight(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let knight = spawn_monster(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('k'),
        Some(5),
        "Knight",
        35,
        8,
        3,
    )
    .with(components::Inventory::new_empty())
    .with(components::BodyParts::default_humanoid())
    .build();
    spawn_item_in_eq(ecs, knight, "Chain armor".to_string(), x, y, level);
    spawn_item_in_eq(ecs, knight, "Zweihander".to_string(), x, y, level);
    knight
}

pub fn spawn_small_slime(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('b'),
        Some(8),
        "Small slime",
        8,
        3,
        2,
    )
    .build()
}

pub fn spawn_slime(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let slime = spawn_monster(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('B'),
        Some(7),
        "Slime",
        50,
        8,
        5,
    )
    .with(components::SpawnsAfterDeath {
        spawns: vec![
            SpawnEntry::new("Small slime".to_string(), 3, 5),
            SpawnEntry::new("Slime".to_string(), 1, 1).with_chance(10),
        ],
    })
    .with(components::Inventory::new_empty())
    .build();
    spawn_item_into_inventory(ecs, slime, "Leather boots".to_string(), x, y, level);
    slime
}

pub fn spawn_mighty_slime(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let slime = spawn_monster(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('B'),
        Some(7),
        "Mighty slime",
        300,
        10,
        3,
    )
    .with(components::SpawnsAfterDeath {
        spawns: vec![SpawnEntry::new("Slime".to_string(), 2, 4)],
    })
    .with(components::FinalBoss {})
    .build();
    slime
}

#[allow(clippy::too_many_arguments)]
fn spawn_monster<S: ToString>(
    ecs: &mut World,
    x: usize,
    y: usize,
    level: usize,
    glyph: rltk::FontCharType,
    texture_index: Option<usize>,
    name: S,
    hp: i32,
    atk: i32,
    def: i32,
) -> EntityBuilder {
    ecs.create_entity()
        .with(components::Position { x, y, level })
        .with(components::Renderable {
            ascii: glyph,
            texture: texture_index,
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
