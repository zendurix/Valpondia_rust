use crate::{
    ecs::{components, State},
    gamelog::GameLog,
};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::base::Dir;

use super::movement::try_move_player;

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
    PickUpItem,

    UnhandledInput, // NoInput
    NoInput,        // shouldnt bbe use (input should be Option<InputYpe)
}

pub fn get_input(gs: &mut State, ctx: &mut Rltk) {
    let mut players = gs.ecs.write_storage::<components::Player>();
    let movables = gs.ecs.read_storage::<components::Movable>();
    for (player, _) in (&mut players, &movables).join() {
        player.input = match ctx.key {
            Some(key) => match key {
                VirtualKeyCode::Numpad1 => Some(InputType::DownLeft),
                VirtualKeyCode::Numpad2 | VirtualKeyCode::Down => Some(InputType::Down),
                VirtualKeyCode::Numpad3 => Some(InputType::DownRight),
                VirtualKeyCode::Numpad4 | VirtualKeyCode::Left => Some(InputType::Left),
                VirtualKeyCode::Numpad5 => Some(InputType::Center),
                VirtualKeyCode::Numpad6 | VirtualKeyCode::Right => Some(InputType::Right),
                VirtualKeyCode::Numpad7 => Some(InputType::UpLeft),
                VirtualKeyCode::Numpad8 | VirtualKeyCode::Up => Some(InputType::Up),
                VirtualKeyCode::Numpad9 => Some(InputType::UpRight),

                VirtualKeyCode::G => Some(InputType::PickUpItem),

                _ => None, // UnhandledInput
            },
            None => None,
        }
    }
}

/// true, if input was handled, and player turn should end
pub fn try_handle_input(gs: &mut State) -> bool {
    let player = *gs.ecs.fetch::<Entity>();
    let player_inp_is_some = gs
        .ecs
        .read_storage::<components::Player>()
        .get(player)
        .unwrap()
        .input
        .is_some();

    if player_inp_is_some {
        let key;
        {
            let mut inputs = gs.ecs.write_storage::<components::Player>();
            let mut input = inputs.get_mut(player).unwrap();
            key = input.input.unwrap();
            input.input = None;
        }
        match key {
            InputType::Down => try_move_player(gs, Dir::Down),
            InputType::DownRight => try_move_player(gs, Dir::DownRight),
            InputType::DownLeft => try_move_player(gs, Dir::DownLeft),
            InputType::Up => try_move_player(gs, Dir::Up),
            InputType::UpLeft => try_move_player(gs, Dir::UpLeft),
            InputType::UpRight => try_move_player(gs, Dir::UpRight),
            InputType::Left => try_move_player(gs, Dir::Left),
            InputType::Right => try_move_player(gs, Dir::Right),
            // wait one turn
            InputType::Center => true,
            InputType::PickUpItem => try_pick_up_item(&mut gs.ecs),
            _ => false,
        }
    } else {
        false
    }
}

/// true if item was picked up
fn try_pick_up_item(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<rltk::Point>();
    let player = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<components::Item>();
    let positions = ecs.read_storage::<components::Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;

    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x as i32 == player_pos.x && position.y as i32 == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => {
            gamelog
                .entries
                .push("There is nothing here to pick up.".to_string());
            false
        }
        Some(item) => {
            let mut pickup = ecs.write_storage::<components::WantsToPickupItem>();
            pickup
                .insert(
                    *player,
                    components::WantsToPickupItem { who: *player, item },
                )
                .expect("Unable to insert want to pickup");
            true
        }
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
