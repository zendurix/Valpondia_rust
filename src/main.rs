#![allow(clippy::new_without_default)]

#[macro_use]
extern crate lazy_static;

pub mod base;
pub mod ecs;
pub mod errors;
pub mod gamelog;
pub mod graphics;
pub mod levels;
pub mod maps;
pub mod rng;

use std::collections::HashSet;

use ecs::{components, game_state::RunState, State};
use levels::level::LevelType;

use maps::MapGenerator;
use specs::prelude::*;

use rltk::RGB;

const WINDOW_WIDTH: usize = 100;
const WINDOW_HEIGHT: usize = 80;

fn main() {
    let context = graphics::create_window(WINDOW_WIDTH, WINDOW_HEIGHT);
    // caves of qud effect
    // context.with_post_scanlines(true);

    let mut gs = State::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    gs.register_all_components();

    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["  =====WELCOME INTO VALPONDIA======  ".to_string()],
    });

    let map_height = WINDOW_HEIGHT - graphics::CONSOLE_BOX_HEIGHT;

    let test = gs.create_new_level(LevelType::BasicDungeon, WINDOW_WIDTH, map_height);

    match test {
        Ok(_) => (),
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
    gs.set_level_as_curent(0);

    let mut p_x = 0;
    let mut p_y = 0;
    while gs.current_map().tile_at_xy(p_x, p_y).blocks_visibility() {
        p_x = rng::range(2, gs.current_map().width_max() as i32 - 2) as usize;
        p_y = rng::range(2, gs.current_map().height_max() as i32 - 2) as usize;
    }

    gs.ecs.insert(rltk::Point::new(p_x, p_y));
    let player = gs
        .ecs
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
        .build();

    gs.ecs.insert(player);

    let rooms = gs.current_map().rooms.clone();
    for room in rooms.iter() {
        let (x, y) = room.center();
        let rand = rng::rand_bool();
        gs.ecs
            .create_entity()
            .with(components::Position { x, y, level: 0 })
            .with(components::Renderable {
                ascii: if rand {
                    rltk::to_cp437('g')
                } else {
                    rltk::to_cp437('o')
                },
                texture: None,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(components::Name {
                name: if rand {
                    "goblin".to_string()
                } else {
                    "orc".to_string()
                },
            })
            .with(components::View {
                visible_tiles: HashSet::<rltk::Point>::new(),
                range: 10,
                should_update: true,
            })
            .with(components::AI {})
            .with(components::BlocksTile {})
            .with(if rand {
                components::Hp { max_hp: 8, hp: 8 }
            } else {
                components::Hp { max_hp: 35, hp: 35 }
            })
            .with(if rand {
                components::CombatBaseStats {
                    attack: 5,
                    defense: 0,
                }
            } else {
                components::CombatBaseStats {
                    attack: 14,
                    defense: 2,
                }
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
