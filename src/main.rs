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
pub mod spawner;

use ecs::{components, game_state::RunState, State};
use kira::{
    instance::InstanceSettings,
    manager::{AudioManager, AudioManagerSettings},
    sound::SoundSettings,
};
use levels::level::LevelType;

use maps::MapGenerator;

use crate::{graphics::GuiDrawer, spawner::player::spawn_player};

const WINDOW_WIDTH: usize = 100;
const WINDOW_HEIGHT: usize = 80;
pub const CONSOLE_BOX_HEIGHT: usize = 8;

fn main() {
    // music
    let mut audio_manager = AudioManager::new(AudioManagerSettings::default()).unwrap();

    if let Ok(mut sound_handle) =
        audio_manager.load_sound("resources/music/Cave.ogg", SoundSettings::default())
    {
        // sound_handle.play(InstanceSettings::default()).unwrap();
    } else {
        println!("Music file not found!");
    }

    let context = graphics::create_window(WINDOW_WIDTH, WINDOW_HEIGHT);
    // caves of qud effect
    // context.with_post_scanlines(true);

    let gui_drawer = GuiDrawer::new(WINDOW_WIDTH, WINDOW_HEIGHT, CONSOLE_BOX_HEIGHT);

    let map_height = WINDOW_HEIGHT - CONSOLE_BOX_HEIGHT;

    let mut gs = State::new(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WINDOW_WIDTH,
        map_height,
        gui_drawer,
    );
    gs.register_all_components();

    gs.ecs.insert(RunState::MainMenu);
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["  =====WELCOME INTO VALPONDIA======  ".to_string()],
    });

    let test = gs.create_new_level(LevelType::BasicDungeon, WINDOW_WIDTH, map_height, 0, None);

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
    let player = spawn_player(&mut gs.ecs, p_x, p_y);

    gs.ecs.insert(player);

    let result = rltk::main_loop(context, gs);
    match result {
        Ok(_) => (),
        Err(e) => {
            println!("ERROR in main loop: {}", e);
            std::process::exit(1);
        }
    }
}
