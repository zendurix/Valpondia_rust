use crate::{
    ecs::{components, game_state::RunState, State},
    gamelog::GameLog,
    levels::level::Level,
    maps::TileType,
};

use specs::prelude::*;

use crate::base::Dir;

pub fn try_move_player(gs: &mut State, move_dir: Dir) -> RunState {
    let mut positions = gs.ecs.write_storage::<components::Position>();
    let mut views = gs.ecs.write_storage::<components::View>();
    let mut views_memories = gs.ecs.write_storage::<components::ViewMemory>();
    let _combat_stats = gs.ecs.read_storage::<components::CombatBaseStats>();
    let hps = gs.ecs.read_storage::<components::Hp>();
    let mut wants_to_melee = gs.ecs.write_storage::<components::WantsToMeleeAtack>();

    let player = *gs.ecs.fetch_mut::<Entity>();
    let mut gamelog = gs.ecs.fetch_mut::<GameLog>();
    let map = &gs.ecs.fetch::<Level>().map;

    let mut player_pos_res = gs.ecs.write_resource::<rltk::Point>();
    let mut pos = positions.get_mut(player).unwrap();
    let mut view = views.get_mut(player).unwrap();
    let mut view_mem = views_memories.get_mut(player).unwrap();

    let mut try_x = pos.x;
    let mut try_y = pos.y;

    match move_dir {
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

    try_x = try_x.min(map.width_max());
    try_y = try_y.min(map.height_max());

    let destination_idx = map.xy_to_index(try_x, try_y);

    if try_x == pos.x && try_y == pos.y {
        // not move (move to same pos)
        gamelog
            .entries
            .push("Player tried to move to same position".to_string());
        return RunState::AwaitingInput;
    }

    for potential_target in map.tile_content[destination_idx].iter() {
        let target = hps.get(*potential_target);
        if let Some(_target) = target {
            wants_to_melee
                .insert(
                    player,
                    components::WantsToMeleeAtack {
                        target: *potential_target,
                    },
                )
                .expect("Add target failed");
            return RunState::PlayerTurn;
        }
    }

    if map.tile_at_xy(try_x, try_y).blocks_movement() {
        gamelog
            .entries
            .push("Something blocks your movement".to_string());
        return RunState::AwaitingInput;
    }

    pos.x = try_x;
    pos.y = try_y;
    view.should_update = true;
    view_mem.should_update = true;
    *player_pos_res = rltk::Point::new(pos.x, pos.y);

    RunState::PlayerTurn
}

pub fn try_move_player_down_level(gs: &mut State) -> RunState {
    let mut positions = gs.ecs.write_storage::<components::Position>();
    let mut views = gs.ecs.write_storage::<components::View>();
    let mut views_memories = gs.ecs.write_storage::<components::ViewMemory>();

    let player = *gs.ecs.fetch_mut::<Entity>();
    let mut gamelog = gs.ecs.fetch_mut::<GameLog>();
    let level = &gs.ecs.fetch::<Level>();

    //let mut player_pos_res = gs.ecs.write_resource::<rltk::Point>();
    let mut pos = positions.get_mut(player).unwrap();
    let mut view = views.get_mut(player).unwrap();
    let mut view_mem = views_memories.get_mut(player).unwrap();

    let index = level.map.xy_to_index(pos.x, pos.y);
    if level.map.tiles[index] == TileType::StairsDown {
        gamelog
            .entries
            .push("You are going down stairs".to_string());

        pos.level = level.level_index + 1;

        view.should_update = true;
        view_mem.should_update = true;

        RunState::MoveLevel(level.level_index + 1)
    } else {
        gamelog.entries.push("There is no stairs down".to_string());
        RunState::AwaitingInput
    }
}

pub fn try_move_player_up_level(gs: &mut State) -> RunState {
    let mut positions = gs.ecs.write_storage::<components::Position>();
    let mut views = gs.ecs.write_storage::<components::View>();
    let mut views_memories = gs.ecs.write_storage::<components::ViewMemory>();

    let player = *gs.ecs.fetch_mut::<Entity>();
    let mut gamelog = gs.ecs.fetch_mut::<GameLog>();
    let level = &gs.ecs.fetch::<Level>();

    // let mut player_pos_res = gs.ecs.write_resource::<rltk::Point>();
    let mut pos = positions.get_mut(player).unwrap();
    let mut view = views.get_mut(player).unwrap();
    let mut view_mem = views_memories.get_mut(player).unwrap();

    let index = level.map.xy_to_index(pos.x, pos.y);
    if level.map.tiles[index] == TileType::StairsUp {
        gamelog.entries.push("You are going up stairs".to_string());

        pos.level = level.level_index - 1;

        view.should_update = true;
        view_mem.should_update = true;

        RunState::MoveLevel(level.level_index - 1)
    } else {
        gamelog.entries.push("There is no stairs up".to_string());
        RunState::AwaitingInput
    }
}
