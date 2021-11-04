use crate::{
    ecs::{components, State},
    maps::Map,
};

use specs::prelude::*;

use crate::base::Dir;

pub fn move_player(gs: &mut State) {
    let mut positions = gs.ecs.write_storage::<components::Position>();
    let movables = gs.ecs.read_storage::<components::Movable>();

    let mut views = gs.ecs.write_storage::<components::View>();
    let mut views_memories = gs.ecs.write_storage::<components::ViewMemory>();
    let mut player_pos = gs.ecs.write_resource::<rltk::Point>();
    let entities = gs.ecs.entities();
    let players = gs.ecs.read_storage::<components::Player>();
    let _combat_stats = gs.ecs.read_storage::<components::CombatBaseStats>();
    let hps = gs.ecs.read_storage::<components::Hp>();
    let mut wants_to_melee = gs.ecs.write_storage::<components::WantsToMeleeAtack>();
    let map = gs.ecs.fetch::<Map>();

    for (entity, _p, mov, pos) in (&entities, &players, &movables, &mut positions).join() {
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

        try_x = try_x.min(map.width_max());
        try_y = try_y.min(map.height_max());

        let destination_idx = map.xy_to_index(try_x, try_y);

        if try_x == pos.x && try_y == pos.y {
            // not move (move to same pos)
            return;
        }
        for potential_target in map.tile_content[destination_idx].iter() {
            let target = hps.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee
                    .insert(
                        entity,
                        components::WantsToMeleeAtack {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return;
            }
        }

        if !map.tile_at_xy(try_x, try_y).blocks_movement() {
            pos.x = try_x;
            pos.y = try_y;
        }

        if let Some(view) = views.get_mut(entity) {
            view.should_update = true;
        }
        if let Some(view_memory) = views_memories.get_mut(entity) {
            view_memory.should_update = true;
        }
        if let Some(_player) = players.get(entity) {
            gs.player_pos = components::Position {
                x: pos.x,
                y: pos.y,
                level: pos.level,
            };
            *player_pos = rltk::Point::new(pos.x, pos.y);
        }
    }
}
