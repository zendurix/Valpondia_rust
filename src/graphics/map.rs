use rltk::{Rltk, RGB};

use crate::levels::map::{Map, TileType};

pub fn draw_map(map: &Map, ctx: &mut Rltk) {
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
