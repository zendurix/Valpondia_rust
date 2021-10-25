pub mod base;
pub mod ecs;
pub mod errors;
pub mod graphics;
pub mod levels;
pub mod maps;

use ecs::State;
use levels::level::LevelType;
use levels::level_manager;
use levels::level_manager::LevelManager;
use maps::Map;

use maps::generators::cellular_automata::CAMapGen;
use maps::generators::cellular_automata::CAMapGenConfig;
use maps::MapGenerator;
use specs::prelude::*;
use specs::World;

use rltk::{Console, GameState, Rltk, RGB};

fn main() {
    let context = rltk::RltkBuilder::simple(100, 100)
        .unwrap()
        .with_title("Valpondia")
        .build()
        .unwrap();

    let mut gs = State::new();
    gs.register_all_components();

    let test = gs.create_new_level(LevelType::Cave, 100, 100);

    match test {
        Ok(m) => (),
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    }

    let map = match CAMapGen::new(100, 100) {
        Ok(map_gen) => match map_gen.generate() {
            Ok(m) => m,
            Err(e) => {
                println!("ERROR: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    };

    gs.ecs.insert(map);

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

    //   for i in 0..40 {
    //       gs.ecs
    //           .create_entity()
    //           .with(ecs::Position {
    //               x: i,
    //               y: i,
    //               level: 0,
    //           })
    //           .with(ecs::Movable { move_dir: None })
    //           .with(ecs::AI {})
    //           .with(ecs::Renderable {
    //               ascii: rltk::to_cp437('☺'),
    //               texture: None,
    //               fg: RGB::named(rltk::RED),
    //               bg: RGB::named(rltk::BLACK),
    //           })
    //           .build();
    //   }

    rltk::main_loop(context, gs);
}
