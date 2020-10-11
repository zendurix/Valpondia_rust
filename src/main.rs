extern crate specs;

mod ecs;

use ecs::State;

use specs::prelude::*;
use specs::World;

use rltk::{Console, GameState, Rltk, RGB};

fn main() {
    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .unwrap();
    let mut gs = State::new();
    gs.register_all_components();

    gs.ecs
        .create_entity()
        .with(ecs::Position {
            x: 40,
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

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(ecs::Position {
                x: i * 7,
                y: 20,
                level: 0,
            })
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
