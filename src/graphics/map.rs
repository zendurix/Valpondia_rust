use rltk::{Rltk, RGB};
use specs::{Join, WorldExt};

use crate::{
    ecs::{components, State},
    maps::{Map, TileType},
};

pub fn draw_map_with_fov(gs: &State, ctx: &mut Rltk) {
    let views = gs.ecs.read_storage::<components::View>();
    let views_memories = gs.ecs.read_storage::<components::ViewMemory>();
    let players = gs.ecs.read_storage::<components::Player>();
    let map = gs.current_map();

    for (view, view_memory, _player) in (&views, &views_memories, &players).join() {
        for pos in view.visible_tiles.iter() {
            let x = pos.x;
            let y = pos.y;
            let tile = map.tile_at_xy(x as usize, y as usize);

            match tile {
                TileType::Floor => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0.0, 1.0, 0.0),
                        RGB::from_f32(0., 0., 0.),
                        rltk::to_cp437('.'),
                    );
                }
                TileType::Wall => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0.0, 1.0, 0.0),
                        RGB::from_f32(0., 0., 0.),
                        rltk::to_cp437('#'),
                    );
                }
            }
        }
        for pos in view_memory
            .seen_tiles
            .symmetric_difference(&view.visible_tiles)
        {
            let x = pos.x;
            let y = pos.y;
            let tile = map.tile_at_xy(x as usize, y as usize);

            match tile {
                TileType::Floor => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0.5, 0.5, 0.5),
                        RGB::from_f32(0., 0., 0.),
                        rltk::to_cp437('.'),
                    );
                }
                TileType::Wall => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0.5, 0.5, 0.5),
                        RGB::from_f32(0., 0., 0.),
                        rltk::to_cp437('#'),
                    );
                }
            }
        }
    }
}

pub fn draw_map_without_fov(map: &Map, ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.tiles().iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }
        x += 1;
        if x > map.width - 1 {
            y += 1;
            x = 0;
        }
    }
}
