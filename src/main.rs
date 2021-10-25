#![allow(clippy::new_without_default)]

pub mod base;
pub mod ecs;
pub mod errors;
pub mod graphics;
pub mod levels;
pub mod maps;

use ecs::State;
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

    let test = gs.create_new_level(LevelType::Cave, 100, 100);

    match test {
        Ok(_) => (),
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    }

    gs.ecs
        .create_entity()
        .with(ecs::Player { input: None })
        .with(ecs::Movable { move_dir: None })
        .with(ecs::Position {
            x: 20,
            y: 25,
            level: 0,
        })
        .with(ecs::Renderable {
            ascii: rltk::to_cp437('@'),
            texture: None,
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    for i in 2..5 {
        gs.ecs
            .create_entity()
            .with(ecs::Position {
                x: i,
                y: i,
                level: 0,
            })
            .with(ecs::Movable { move_dir: None })
            .with(ecs::AI {})
            .with(ecs::Renderable {
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
