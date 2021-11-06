use std::collections::HashSet;

use rltk::RGB;
use specs::{Builder, Entity, World, WorldExt};

use crate::{ecs::components, rng};

pub fn spawn_random_monster(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    let rand = rng::roll_dice(1, 4);

    match rand {
        1 => spawn_goblin(ecs, x, y, level),
        2 => spawn_orc(ecs, x, y, level),
        3 => spawn_human(ecs, x, y, level),
        4 => spawn_knight(ecs, x, y, level),
        _ => panic!("Wrong random number during monster spawning"),
    }
}

pub fn spawn_goblin(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(ecs, x, y, level, rltk::to_cp437('g'), "goblin", 10, 4, 1)
}
pub fn spawn_orc(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(ecs, x, y, level, rltk::to_cp437('o'), "orc", 32, 12, 3)
}
pub fn spawn_human(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(ecs, x, y, level, rltk::to_cp437('h'), "human", 20, 15, 2)
}
pub fn spawn_knight(ecs: &mut World, x: usize, y: usize, level: usize) -> Entity {
    spawn_monster(ecs, x, y, level, rltk::to_cp437('k'), "knight", 35, 8, 7)
}

#[allow(clippy::too_many_arguments)]
fn spawn_monster<S: ToString>(
    ecs: &mut World,
    x: usize,
    y: usize,
    level: usize,
    glyph: rltk::FontCharType,
    name: S,
    hp: i32,
    atk: i32,
    def: i32,
) -> Entity {
    ecs.create_entity()
        .with(components::Position { x, y, level })
        .with(components::Renderable {
            ascii: glyph,
            texture: None,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(components::Name {
            name: name.to_string(),
        })
        .with(components::View {
            visible_tiles: HashSet::<rltk::Point>::new(),
            range: 10,
            should_update: true,
        })
        .with(components::AI {})
        .with(components::BlocksTile {})
        .with(components::Hp { max_hp: hp, hp })
        .with(components::CombatBaseStats {
            attack: atk,
            defense: def,
        })
        .build()
}
