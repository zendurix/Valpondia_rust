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
        RGB::named(rltk::WHITE),
        "Dagger",
    )
    .with(components::MeleeDamageBonus { power: 3 })
    .with(components::Equippable {
        body_part: BodyPart::OneHanded,
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
        RGB::named(rltk::WHITE),
        "Zweihander",
    )
    .with(components::MeleeDamageBonus { power: 10 })
    .with(components::Equippable {
        body_part: BodyPart::TwoHanded,
    })
    .build()
}
