use crate::ecs::{components, State};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::base::Dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Center,

    UnhandledInput, // NoInput
    NoInput,        // shouldnt bbe use (input should be Option<InputYpe)
}

pub fn get_input(gs: &mut State, ctx: &mut Rltk) {
    let mut players = gs.ecs.write_storage::<components::Player>();
    let movables = gs.ecs.read_storage::<components::Movable>();
    for (player, _) in (&mut players, &movables).join() {
        player.input = match ctx.key {
            Some(key) => match key {
                VirtualKeyCode::Numpad1 | VirtualKeyCode::B => Some(InputType::DownLeft),
                VirtualKeyCode::Numpad2 | VirtualKeyCode::Down | VirtualKeyCode::J => {
                    Some(InputType::Down)
                }
                VirtualKeyCode::Numpad3 | VirtualKeyCode::N => Some(InputType::DownRight),
                VirtualKeyCode::Numpad4 | VirtualKeyCode::Left | VirtualKeyCode::H => {
                    Some(InputType::Left)
                }
                VirtualKeyCode::Numpad5 => Some(InputType::Center),
                VirtualKeyCode::Numpad6 | VirtualKeyCode::Right | VirtualKeyCode::L => {
                    Some(InputType::Right)
                }
                VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => Some(InputType::UpLeft),
                VirtualKeyCode::Numpad8 | VirtualKeyCode::Up | VirtualKeyCode::K => {
                    Some(InputType::Up)
                }
                VirtualKeyCode::Numpad9 => Some(InputType::UpRight),

                _ => None, // UnhandledInput
            },
            None => None,
        }
    }
}

pub fn handle_input(gs: &mut State) {
    let mut players = gs.ecs.write_storage::<components::Player>();
    let mut movables = gs.ecs.write_storage::<components::Movable>();
    for (player, mov) in (&mut players, &mut movables).join() {
        mov.move_dir = match player.input {
            Some(key) => match key {
                InputType::DownLeft => Some(Dir::DownLeft),
                InputType::Down => Some(Dir::Down),
                InputType::DownRight => Some(Dir::DownRight),
                InputType::Left => Some(Dir::Left),
                InputType::Center => Some(Dir::Center),
                InputType::Right => Some(Dir::Right),
                InputType::UpLeft => Some(Dir::UpLeft),
                InputType::Up => Some(Dir::Up),
                InputType::UpRight => Some(Dir::UpRight),
                _ => None,
            },
            None => None,
        };        
        player.input = None;
    }
}

/*
/// VIM CONTROLS
///
///
///
///
///

 y k u    7 8 9
  \|/      \|/
 h-+-l    4-5-6
  /|\      /|\
 b j n    1 2 3
vi-keys   numpad



                VirtualKeyCode::Numpad1 | VirtualKeyCode::B => Some(InputType::DownLeft),
                VirtualKeyCode::Numpad2 | VirtualKeyCode::Down | VirtualKeyCode::J => {
                    Some(InputType::Down)
                }
                VirtualKeyCode::Numpad3 | VirtualKeyCode::N => Some(InputType::DownRight),
                VirtualKeyCode::Numpad4 | VirtualKeyCode::Left | VirtualKeyCode::H => {
                    Some(InputType::Left)
                }
                VirtualKeyCode::Numpad5 => Some(InputType::Center),
                VirtualKeyCode::Numpad6 | VirtualKeyCode::Right | VirtualKeyCode::L => {
                    Some(InputType::Right)
                }
                VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => Some(InputType::UpLeft),
                VirtualKeyCode::Numpad8 | VirtualKeyCode::Up | VirtualKeyCode::K => {
                    Some(InputType::Up)
                }
                VirtualKeyCode::Numpad9 => Some(InputType::UpRight),


*/
