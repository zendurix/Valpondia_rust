use crate::ecs::{Movable, Player, State};

use rltk::{Console, GameState, Rltk, RGB};
use specs::{prelude::*, Component};

use crate::base::{Dir, Key};

/// Pauses game, until some input is providen.
pub fn get_input(gs: &mut State, ctx: &mut Rltk) {
    let mut players = gs.ecs.write_storage::<Player>();
    let movables = gs.ecs.read_storage::<Movable>();
    for (player, _) in (&mut players, &movables).join() {
        //while ctx.key.is_none() {}
        player.input = ctx.key;
    }
}

pub fn handle_input(gs: &mut State, ctx: &mut Rltk) {
    let mut players = gs.ecs.read_storage::<Player>();
    let mut movables = gs.ecs.write_storage::<Movable>();
    for (player, mov) in (&players, &mut movables).join() {
        mov.move_dir = match player.input {
            Some(key) => match key {
                Key::Numpad1 => Some(Dir::DownLeft),
                Key::Numpad2 => Some(Dir::Down),
                Key::Numpad3 => Some(Dir::DownRight),
                Key::Numpad4 => Some(Dir::Left),
                Key::Numpad5 => Some(Dir::Center),
                Key::Numpad6 => Some(Dir::Right),
                Key::Numpad7 => Some(Dir::UpLeft),
                Key::Numpad8 => Some(Dir::Up),
                Key::Numpad9 => Some(Dir::UpRight),
                _ => None,
            },
            None => None,
        }
    }
}
