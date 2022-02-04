use std::collections::{HashMap, HashSet};

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
            seen_tiles: HashMap::default(),
            should_update: true,
        })
        .with(components::Position { x, y, level: 0 })
        .with(components::Name {
            name: "player".to_string(),
        })
        .with(components::Renderable {
            ascii: rltk::to_cp437('@'),
            texture: Some(2),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(components::Hp {
            max_hp: 300,
            hp: 300,
        })
        .with(components::CombatBaseStats {
            attack: 500,
            defense: 1,
        })
        .with(components::BodyParts::default_humanoid())
        .with(components::Inventory::new_empty())
        .build()
}
