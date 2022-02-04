use itertools::Itertools;
use rltk::{Rltk, RGB};
use specs::{Entity, Join, WorldExt};

use crate::{
    ecs::{components, State},
    levels::level::Level,
    maps::TileType,
};

use super::window::SPRITE_16X16_CONSOLE_INDEX;

pub fn draw_entities(gs: &State, _ctx: &mut Rltk) {
    let mut draw_batch = rltk::DrawBatch::new();
    draw_batch.target(SPRITE_16X16_CONSOLE_INDEX);

    let positions = gs.ecs.read_storage::<components::Position>();
    let renderables = gs.ecs.read_storage::<components::Renderable>();

    let current_level = gs.ecs.fetch::<Level>();

    let entites = gs.ecs.entities();
    let views = gs.ecs.read_storage::<components::View>();
    let player = *gs.ecs.fetch::<Entity>();
    let player_view = views.get(player).unwrap();

    for (ent, pos, render) in (&entites, &positions, &renderables)
        .join()
        .sorted_by(|a, b| Ord::cmp(&b.2.render_order, &a.2.render_order))
    {
        let index = current_level.map.xy_to_index(pos.x, pos.y);
        if pos.level == gs.current_level
            && player_view
                .visible_tiles
                .contains(&rltk::Point::new(pos.x, pos.y))
            && (ent == player
                || (current_level.map.tiles[index] != TileType::StairsUp
                    && current_level.map.tiles[index] != TileType::StairsDown))
        {
            //ctx.set(pos.x, pos.y, render.fg, render.bg, render.ascii);

            draw_batch.set(
                rltk::Point::new(pos.x, pos.y),
                rltk::ColorPair::new(RGB::from_f32(1.0, 1., 1.0), RGB::from_f32(0., 0., 0.)),
                render.texture.unwrap_or(3),
            );
        }
    }
}
