extern crate rand;
extern crate specs;

pub mod base;
pub mod data;
pub mod ecs;
pub mod graphics;

use data::map::Map;
use ecs::State;

use specs::prelude::*;
use specs::World;

use rltk::{Console, GameState, Rltk, RGB};

fn main() {
    let context = rltk::RltkBuilder::simple(40, 40)
        .unwrap()
        .with_title("Valpondia")
        .build()
        .unwrap();

    let mut gs = State::new();
    gs.register_all_components();

    let map = Map::new(40, 40).with_edges_solid();
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

    for i in 0..40 {
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

    rltk::main_loop(context, gs);
}
