use rltk::RGB;
use specs::{Builder, Entity, EntityBuilder, World, WorldExt};

use crate::ecs::components;

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

pub fn spawn_magic_missile_scroll(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437(')'),
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

/// aoe spell (actuallly granade :) )
pub fn spawn_fireball_scroll(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    create_base_item_components(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437(')'),
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

#[allow(clippy::too_many_arguments)]
fn create_base_item_components<S: ToString>(
    ecs: &mut World,
    x: usize,
    y: usize,
    level: usize,
    glyph: rltk::FontCharType,
    color: rltk::RGB,
    name: S,
) -> EntityBuilder {
    ecs.create_entity()
        .with(components::Item {})
        .with(components::Position { x, y, level })
        .with(components::Renderable {
            ascii: glyph,
            texture: None,
            fg: color,
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(components::Name {
            name: name.to_string(),
        })
}
