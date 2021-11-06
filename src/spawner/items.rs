use rltk::RGB;
use specs::{Builder, Component, Entity, World, WorldExt};

use crate::ecs::components;

pub fn spawn_healing_potion(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_item(
        ecs,
        x,
        y,
        level,
        rltk::to_cp437('i'),
        RGB::named(rltk::PINK),
        "Health potion",
        vec![components::Heal { heal_power: 20 }],
    )
}

#[allow(clippy::too_many_arguments)]
fn spawn_item<S: ToString, C: Component + Send + Sync>(
    ecs: &mut World,
    x: usize,
    y: usize,
    level: usize,
    glyph: rltk::FontCharType,
    color: rltk::RGB,
    name: S,
    components: Vec<C>,
) -> Entity {
    let mut item = ecs
        .create_entity()
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
        });
    for comp in components {
        item = item.with(comp);
    }
    item.build()
}
