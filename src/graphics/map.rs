use rltk::{DrawBatch, Rltk, RGB};
use specs::{Join, WorldExt};

use crate::{
    ecs::{components, State},
    levels::level::Level,
    maps::Map,
};

use super::window::SPRITE_CONSOLE_INDEX;

pub fn draw_map_with_fov(gs: &State, _ctx: &mut Rltk) {
    let views = gs.ecs.read_storage::<components::View>();
    let views_memories = gs.ecs.read_storage::<components::ViewMemory>();
    let players = gs.ecs.read_storage::<components::Player>();

    let current_level = gs.ecs.fetch::<Level>();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(SPRITE_CONSOLE_INDEX);

    for (view, view_memory, _player) in (&views, &views_memories, &players).join() {
        for pos in view.visible_tiles.iter() {
            let x = pos.x;
            let y = pos.y;
            let tile = current_level.map.tile_at_xy(x as usize, y as usize);

            draw_batch.set(
                rltk::Point::new(x, y),
                rltk::ColorPair::new(RGB::from_f32(1.0, 1., 1.0), RGB::from_f32(0., 0., 0.)),
                tile.texture_index().unwrap(),
            );
        }

        if let Some(tiles) = view_memory.seen_tiles.get(&current_level.level_index) {
            for pos in tiles.symmetric_difference(&view.visible_tiles) {
                let x = pos.x;
                let y = pos.y;
                let tile = current_level.map.tile_at_xy(x as usize, y as usize);

                draw_batch.set(
                    rltk::Point::new(x, y),
                    rltk::ColorPair::new(RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.)),
                    tile.texture_index().unwrap(),
                );
            }
        }
    }
}

pub fn draw_map_without_fov(map: &Map, _ctx: &mut Rltk) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(SPRITE_CONSOLE_INDEX);

    let mut x = 0;
    let mut y = 0;
    for tile in map.tiles().iter() {
        draw_batch.set(
            rltk::Point::new(x, y),
            rltk::ColorPair::new(RGB::from_f32(1.0, 1., 1.0), RGB::from_f32(0., 0., 0.)),
            tile.texture_index().unwrap(),
        );

        x += 1;
        if x > map.width - 1 {
            y += 1;
            x = 0;
        }
    }
}
