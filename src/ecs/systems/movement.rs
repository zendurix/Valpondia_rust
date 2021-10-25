use crate::{
    ecs::{components, State},
    maps::TileType,
};

use rltk::Rltk;
use specs::prelude::*;

use crate::base::Dir;

pub fn move_all(gs: &mut State, _ctx: &mut Rltk) {
    let mut positions = gs.ecs.write_storage::<components::Position>();
    let movables = gs.ecs.read_storage::<components::Movable>();

    let mut views = gs.ecs.write_storage::<components::View>();
    let mut views_memories = gs.ecs.write_storage::<components::ViewMemory>();
    let entities = gs.ecs.entities();

    for (entity, mov, pos) in (&entities, &movables, &mut positions).join() {
        let mut try_x = pos.x;
        let mut try_y = pos.y;

        if let Some(dir) = mov.move_dir {
            match dir {
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
                Dir::Center => (),
            }
        }

        let map = gs.current_map();

        try_x = try_x.min(map.width_max());
        try_y = try_y.min(map.height_max());

        if map.tile_at_xy(try_x, try_y) != TileType::Wall {
            pos.x = try_x;
            pos.y = try_y;
        }

        if let Some(view) = views.get_mut(entity) {
            view.should_update = true;
        }
        if let Some(view_memory) = views_memories.get_mut(entity) {
            view_memory.should_update = true;
        }
    }
}
