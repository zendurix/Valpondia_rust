#![allow(clippy::new_without_default)]

#[macro_use]
extern crate lazy_static;

pub mod base;
pub mod ecs;
pub mod errors;
pub mod graphics;
pub mod levels;
pub mod maps;
pub mod rng;

use std::collections::HashSet;

use ecs::{components, State};
use levels::level::LevelType;

use maps::MapGenerator;
use specs::prelude::*;

use rltk::RGB;

fn main() {
    let context = graphics::create_window(100, 100);
    // caves of qud effect
    // context.with_post_scanlines(true);

    let mut gs = State::new();
    gs.register_all_components();

    let test = gs.create_new_level(LevelType::BasicDungeon, 100, 100);

    match test {
        Ok(_) => (),
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    }

    let mut p_x = 0;
    let mut p_y = 0;
    while gs.current_map().tile_at_xy(p_x, p_y).blocks_visibility() {
        p_x = rng::range(2, gs.current_map().width_max() as i32 - 2) as usize;
        p_y = rng::range(2, gs.current_map().height_max() as i32 - 2) as usize;
    }

    gs.ecs
        .create_entity()
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
        .with(components::Position {
            x: p_x,
            y: p_y,
            level: 0,
        })
        .with(components::Renderable {
            ascii: rltk::to_cp437('@'),
            texture: None,
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    for i in 2..5 {
        gs.ecs
            .create_entity()
            .with(ecs::components::Position {
                x: i,
                y: i,
                level: 0,
            })
            .with(components::Movable { move_dir: None })
            .with(components::AI {})
            .with(components::Renderable {
                ascii: rltk::to_cp437('☺'),
                texture: None,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }

    let result = rltk::main_loop(context, gs);
    match result {
        Ok(_) => (),
        Err(e) => {
            println!("ERROR in main loop: {}", e);
            std::process::exit(1);
        }
    }
}
