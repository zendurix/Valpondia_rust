use itertools::Itertools;
use rltk::{DrawBatch, Rltk, RGB};
use specs::{Entity, Join, WorldExt};

use crate::{
    ecs::{components, State},
    levels::level::Level,
    maps::{Map, TileType},
};

use super::window::SPRITE_CONSOLE_INDEX;

pub const CAMERA_SIZE_X: i32 = 50;
pub const CAMERA_SIZE_Y: i32 = 30;

/// return x_left, x_right, y_up, y_down
fn calculate_camera_bounds(
    pos_x: i32,
    pos_y: i32,
    map_width: i32,
    map_height: i32,
) -> (i32, i32, i32, i32) {
    let cam_length_half = CAMERA_SIZE_X / 2;
    let cam_height_half = CAMERA_SIZE_Y / 2;

    let (x_left, x_right) = if pos_x - cam_length_half <= 0 {
        (0, CAMERA_SIZE_X)
    } else if (pos_x + cam_length_half) > map_width {
        (map_width - CAMERA_SIZE_X, map_width)
    } else {
        (pos_x - cam_length_half, pos_x + cam_length_half)
    };

    let (y_up, y_down) = if pos_y - cam_height_half <= 0 {
        (0, CAMERA_SIZE_Y)
    } else if pos_y + cam_height_half >= map_height {
        (map_height - CAMERA_SIZE_Y, map_height)
    } else {
        (pos_y - cam_height_half, pos_y + cam_height_half)
    };

    (x_left, x_right, y_up, y_down)
}

pub fn draw_map_and_entities_with_fov_and_camera(gs: &State, _ctx: &mut Rltk) {
    let entites = gs.ecs.entities();
    let views = gs.ecs.read_storage::<components::View>();
    let views_memories = gs.ecs.read_storage::<components::ViewMemory>();
    let positions = gs.ecs.read_storage::<components::Position>();
    let renderables = gs.ecs.read_storage::<components::Renderable>();

    let current_level = gs.ecs.fetch::<Level>();
    let player_pos = gs.ecs.fetch::<rltk::Point>();
    let player = *gs.ecs.fetch::<Entity>();
    let player_view = views.get(player).unwrap();
    let player_view_memory = views_memories.get(player).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(SPRITE_CONSOLE_INDEX);

    // camera bounds
    let (x_left, x_right, y_up, y_down) = calculate_camera_bounds(
        player_pos.x,
        player_pos.y,
        current_level.map.width_max() as i32,
        current_level.map.height_max() as i32,
    );

    let seen_tiles_opt = player_view_memory
        .seen_tiles
        .get(&current_level.level_index)
        .and_then(|t| {
            Some(
                t.symmetric_difference(&player_view.visible_tiles)
                    .collect_vec(),
            )
        });

    for i in 0..=(y_down - y_up) {
        for j in 0..=(x_right - x_left) {
            let x = j + x_left;
            let y = i + y_up;
            let point = rltk::Point::new(x, y);

            let tile = current_level.map.tile_at_xy(x as usize, y as usize);

            if player_view.visible_tiles.contains(&point) {
                draw_batch.set(
                    rltk::Point::new(j, i),
                    rltk::ColorPair::new(RGB::from_f32(1.0, 1., 1.0), RGB::from_f32(0., 0., 0.)),
                    tile.texture_index().unwrap(),
                );
            }

            if let Some(seen_tiles) = &seen_tiles_opt {
                if seen_tiles.contains(&&point) {
                    draw_batch.set(
                        rltk::Point::new(j, i),
                        rltk::ColorPair::new(
                            RGB::from_f32(0.5, 0.5, 0.5),
                            RGB::from_f32(0., 0., 0.),
                        ),
                        tile.texture_index().unwrap(),
                    );
                }
            }
            if let Some((ent, pos, rend)) = (&entites, &positions, &renderables)
                .join()
                .sorted_by(|a, b| Ord::cmp(&a.2.render_order, &b.2.render_order))
                .find(|(_e, p, _r)| {
                    rltk::Point::new(p.x, p.y) == point && current_level.level_index == p.level
                })
            {
                if player_view
                    .visible_tiles
                    .contains(&rltk::Point::new(pos.x, pos.y))
                    && (ent == player
                        || (tile != TileType::StairsUp && tile != TileType::StairsDown))
                {
                    draw_batch.set(
                        rltk::Point::new(j, i),
                        rltk::ColorPair::new(
                            RGB::from_f32(1.0, 1., 1.0),
                            RGB::from_f32(0., 0., 0.),
                        ),
                        rend.texture.unwrap_or(3),
                    );
                }
            }
        }
    }
}

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
