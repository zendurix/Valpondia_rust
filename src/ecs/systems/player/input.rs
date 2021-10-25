use crate::ecs::{Movable, Player, State};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::base::Dir;

/// Pauses game, until some input is providen.
pub fn get_input(gs: &mut State, ctx: &mut Rltk) {
    let mut players = gs.ecs.write_storage::<Player>();
    let movables = gs.ecs.read_storage::<Movable>();
    for (player, _) in (&mut players, &movables).join() {
        //while ctx.key.is_none() {}
        player.input = ctx.key;
    }
}

pub fn handle_input(gs: &mut State, _ctx: &mut Rltk) {
    let players = gs.ecs.read_storage::<Player>();
    let mut movables = gs.ecs.write_storage::<Movable>();
    for (player, mov) in (&players, &mut movables).join() {
        mov.move_dir = match player.input {
            Some(key) => match key {
                VirtualKeyCode::Numpad1 => Some(Dir::DownLeft),
                VirtualKeyCode::Numpad2 => Some(Dir::Down),
                VirtualKeyCode::Numpad3 => Some(Dir::DownRight),
                VirtualKeyCode::Numpad4 => Some(Dir::Left),
                VirtualKeyCode::Numpad5 => Some(Dir::Center),
                VirtualKeyCode::Numpad6 => Some(Dir::Right),
                VirtualKeyCode::Numpad7 => Some(Dir::UpLeft),
                VirtualKeyCode::Numpad8 => Some(Dir::Up),
                VirtualKeyCode::Numpad9 => Some(Dir::UpRight),
                _ => None,
            },
            None => None,
        }
    }
}
