use crate::ecs::{Movable, State, AI};

use rand::Rng;
use rltk::Rltk;
use specs::prelude::*;

use crate::base::Dir;

/// ai random movement
pub fn move_all(gs: &mut State, _ctx: &mut Rltk) {
    let ais = gs.ecs.read_storage::<AI>();
    let mut movables = gs.ecs.write_storage::<Movable>();
    for (_ais, mov) in (&ais, &mut movables).join() {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(1..=9);
        mov.move_dir = match rand {
            1 => Some(Dir::DownLeft),
            2 => Some(Dir::Down),
            3 => Some(Dir::DownRight),
            4 => Some(Dir::Left),
            5 => Some(Dir::Center),
            6 => Some(Dir::Right),
            7 => Some(Dir::UpLeft),
            8 => Some(Dir::Up),
            9 => Some(Dir::UpRight),
            _ => None,
        };
    }
}
