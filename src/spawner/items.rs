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
    .with(components::Usable {})
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
        })
        .with(components::Name {
            name: name.to_string(),
        })
}
