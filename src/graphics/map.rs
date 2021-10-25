use rltk::{Rltk, RGB};
use specs::{Join, WorldExt};

use crate::{
    ecs::{components, State},
    maps::{Map, TileType},
};

pub fn draw_map_with_fov(gs: &State, ctx: &mut Rltk) {
    let mut views = gs.ecs.write_storage::<components::View>();
    let mut players = gs.ecs.read_storage::<components::Player>();
    let map = gs.current_map();

    for (view, _player) in (&mut views, &players).join() {
        for pos in view.visible_tiles.iter() {
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
                        RGB::from_f32(0.0, 1.0, 0.0),
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
