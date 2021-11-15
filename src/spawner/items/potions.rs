use rltk::RGB;
use specs::{Builder, Entity, World};

use crate::ecs::components;

use super::create_base_item_components;

pub fn spawn_great_healing_potion(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('i'),
        RGB::named(rltk::RED2),
        "Great health potion",
    )
    .with(components::Heal { heal_power: 50 })
    .with(components::Usable {
        destoyed_on_use: true,
    })
    .build()
}

pub fn spawn_healing_potion(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('i'),
        RGB::named(rltk::PINK),
        "Health potion",
    )
    .with(components::Heal { heal_power: 20 })
    .with(components::Usable {
        destoyed_on_use: true,
    })
    .build()
}
