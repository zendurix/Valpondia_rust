use std::collections::HashSet;

use rltk::RGB;
use specs::{Builder, Entity, World, WorldExt};

use crate::ecs::components;

pub fn spawn_player(ecs: &mut World, x: usize, y: usize) -> Entity {
    ecs.create_entity()
        .with(components::Player { input: None })
        .with(components::Movable { move_dir: None })
        .with(components::View {
            range: 40,
            visible_tiles: HashSet::<rltk::Point>::new(),
            should_update: true,
        })
        .with(components::ViewMemory {
            seen_tiles: HashSet::<rltk::Point>::new(),
            should_update: true,
        })
        .with(components::Position { x, y, level: 0 })
        .with(components::Name {
            name: "hlop".to_string(),
        })
        .with(components::Renderable {
            ascii: rltk::to_cp437('@'),
            texture: None,
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(components::Hp {
            max_hp: 100,
            hp: 100,
        })
        .with(components::CombatBaseStats {
            attack: 10,
            defense: 3,
        })
        .build()
}
