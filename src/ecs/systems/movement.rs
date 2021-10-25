use crate::{
    ecs::{Movable, Position, State},
    maps::TileType,
};

use rltk::{Console, GameState, Rltk, RGB};
use specs::{prelude::*, Component};

use crate::base::Dir;

/// Pauses game, until some input is providen.
pub fn move_all(gs: &mut State, ctx: &mut Rltk) {
    let mut positions = gs.ecs.write_storage::<Position>();
    let movables = gs.ecs.read_storage::<Movable>();

    for (mov, pos) in (&movables, &mut positions).join() {
        let mut try_x = pos.x;
        let mut try_y = pos.y;

        match mov.move_dir {
            Some(dir) => match dir {
                Dir::Up => {
                    try_y -= 1;
                }
                Dir::Down => {
                    try_y += 1;
                }
                Dir::Left => {
                    try_x -= 1;
                }
                Dir::Right => {
                    try_x += 1;
                }
                Dir::UpLeft => {
                    try_y -= 1;
                    try_x -= 1;
                }
                Dir::UpRight => {
                    try_y -= 1;
                    try_x += 1;
                }
                Dir::DownLeft => {
                    try_y += 1;
                    try_x -= 1;
                }
                Dir::DownRight => {
                    try_y += 1;
                    try_x += 1;
                }
                _ => (),
            },
            None => (),
        }

        let map = gs.current_map();

        try_x = try_x.min(map.width_max());
        try_y = try_y.min(map.height_max());

        if map.tile_at_xy(try_x, try_y) != TileType::Wall {
            pos.x = try_x;
            pos.y = try_y;
        }
    }
}
