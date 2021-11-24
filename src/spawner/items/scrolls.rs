use rltk::RGB;
use specs::{Builder, Entity, World};

use crate::ecs::components;

use super::create_base_item_components;

pub fn spawn_magic_missile_scroll(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437(')'),
        Some(15),
        RGB::named(rltk::WHITE),
        "Magic missile scroll",
    )
    .with(components::Usable {
        destoyed_on_use: true,
    })
    .with(components::Ranged { range: 8 })
    .with(components::InflictsDamage { damage: 20 })
    .build()
}

/// aoe spell
pub fn spawn_fireball_scroll(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437(')'),
        Some(15),
        RGB::named(rltk::ORANGE),
        "Fireball scroll",
    )
    .with(components::Usable {
        destoyed_on_use: true,
    })
    .with(components::Ranged { range: 10 })
    .with(components::AreaOfEffect { radius: 4 })
    .with(components::InflictsDamage { damage: 15 })
    .build()
}

pub fn spawn_sleep_scroll(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437(')'),
        Some(15),
        RGB::named(rltk::BROWN2),
        "Sleep scroll",
    )
    .with(components::Usable {
        destoyed_on_use: true,
    })
    .with(components::Ranged { range: 8 })
    .with(components::Sleeping { duration: 5 })
    .build()
}

pub fn spawn_area_sleep_scroll(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437(')'),
        Some(15),
        RGB::named(rltk::PINK),
        "Area sleep scroll",
    )
    .with(components::Usable {
        destoyed_on_use: true,
    })
    .with(components::Ranged { range: 8 })
    .with(components::AreaOfEffect { radius: 4 })
    .with(components::Sleeping { duration: 5 })
    .build()
}

pub fn spawn_teleport_scroll(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437(')'),
        Some(15),
        RGB::named(rltk::BLUE),
        "Teleport scroll",
    )
    .with(components::Usable {
        destoyed_on_use: true,
    })
    .with(components::Ranged { range: 15 })
    .with(components::Teleporting {})
    .build()
}
