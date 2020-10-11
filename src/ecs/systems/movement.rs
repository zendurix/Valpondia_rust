use crate::ecs::{Movable, Position, State};

use rltk::{Console, GameState, Rltk, RGB};
use specs::{prelude::*, Component};

use crate::base::Dir;

/// Pauses game, until some input is providen.
pub fn move_all(gs: &mut State, ctx: &mut Rltk) {
    let mut positions = gs.ecs.write_storage::<Position>();
    let mut movables = gs.ecs.read_storage::<Movable>();
    for (mov, pos) in (&movables, &mut positions).join() {
        match mov.move_dir {
            Some(dir) => match dir {
                Dir::Up => {
                    pos.y -= 1;
                }
                Dir::Down => {
                    pos.y += 1;
                }
                Dir::Left => {
                    pos.x -= 1;
                }
                Dir::Right => {
                    pos.x += 1;
                }
                Dir::UpLeft => {
                    pos.y -= 1;
                    pos.x -= 1;
                }
                Dir::UpRight => {
                    pos.y -= 1;
                    pos.x += 1;
                }
                Dir::DownLeft => {
                    pos.y += 1;
                    pos.x -= 1;
                }
                Dir::DownRight => {
                    pos.y += 1;
                    pos.x += 1;
                }
                _ => (),
            },
            None => (),
        }
    }
}
