use rltk::RGB;
use specs::{Builder, Entity, World};

use crate::{
    ecs::components::{self, BodyPart},
    spawner::items::create_base_item_components,
};

pub fn spawn_dagger(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('/'),
        Some(11),
        RGB::named(rltk::WHITE),
        "Dagger",
    )
    .with(components::MeleeDamageBonus { power: 1 })
    .with(components::Equippable {
        body_part: BodyPart::OneHanded,
    })
    .build()
}

pub fn spawn_shortsword(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('/'),
        Some(11),
        RGB::named(rltk::WHITE),
        "Short sword",
    )
    .with(components::MeleeDamageBonus { power: 2 })
    .with(components::Equippable {
        body_part: BodyPart::OneHanded,
    })
    .build()
}

pub fn spawn_longsword(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('/'),
        Some(12),
        RGB::named(rltk::WHITE),
        "Long sword",
    )
    .with(components::MeleeDamageBonus { power: 4 })
    .with(components::Equippable {
        body_part: BodyPart::TwoHanded,
    })
    .build()
}

pub fn spawn_zweihander(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('/'),
        Some(12),
        RGB::named(rltk::WHITE),
        "Zweihander",
    )
    .with(components::MeleeDamageBonus { power: 6 })
    .with(components::Equippable {
        body_part: BodyPart::TwoHanded,
    })
    .build()
}

pub fn spawn_champion_sword(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('/'),
        Some(12),
        RGB::named(rltk::WHITE),
        "Champion Sword",
    )
    .with(components::MeleeDamageBonus { power: 10 })
    .with(components::Equippable {
        body_part: BodyPart::TwoHanded,
    })
    .build()
}
