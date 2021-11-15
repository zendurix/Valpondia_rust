use rltk::RGB;
use specs::{Builder, Entity, World};

use crate::{
    ecs::components::{self, BodyPart},
    spawner::items::create_base_item_components,
};

pub fn spawn_leather_armor(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('['),
        RGB::named(rltk::BROWN1),
        "Dagger",
    )
    .with(components::DefenseBonus { defense: 2 })
    .with(components::Equippable {
        body_part: BodyPart::Body,
    })
    .build()
}

pub fn spawn_gino_rossi_boots(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('['),
        RGB::named(rltk::BROWN1),
        "Gino rossi boots",
    )
    .with(components::DefenseBonus { defense: 5 })
    .with(components::Equippable {
        body_part: BodyPart::Feets,
    })
    .build()
}

pub fn spawn_chain_armor(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('['),
        RGB::named(rltk::WHITE),
        "Chain armor",
    )
    .with(components::DefenseBonus { defense: 5 })
    .with(components::Equippable {
        body_part: BodyPart::Body,
    })
    .build()
}
