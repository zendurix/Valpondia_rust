use rltk::RGB;
use specs::{Builder, EntityBuilder, World, WorldExt};

use crate::ecs::components;

pub mod eq;
pub mod potions;
pub mod scrolls;

#[allow(clippy::too_many_arguments)]
pub(crate) fn create_base_item_components<S: ToString>(
    ecs: &mut World,
    x: usize,
    y: usize,
    level: usize,
    glyph: rltk::FontCharType,
    texture_index: Option<usize>,
    color: rltk::RGB,
    name: S,
) -> EntityBuilder {
    ecs.create_entity()
        .with(components::Item {})
        .with(components::Position { x, y, level })
        .with(components::Renderable {
            ascii: glyph,
            texture: texture_index,
            fg: color,
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(components::Name {
            name: name.to_string(),
        })
}
