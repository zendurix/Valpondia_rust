use rltk::{Rltk, RGB};
use specs::{Entity, WorldExt};

use crate::{
    ecs::{
        components,
        game_state::TargetingAction,
        systems::player::{input::get_input, InputType},
        State,
    },
    levels::level::Level,
};

#[derive(PartialEq, Copy, Clone)]
pub enum TargetingMenuAction {
    Cancel,
    NoResponse,
    Selected,
}

pub fn show_targeting(
    gs: &mut State,
    ctx: &mut Rltk,
    action: TargetingAction,
) -> TargetingMenuAction {
    let player = *gs.ecs.fetch::<Entity>();
    let player_pos = *gs.ecs.fetch::<rltk::Point>();
    let views = gs.ecs.read_storage::<components::View>();
    let names = gs.ecs.read_storage::<components::Name>();
    let view = views.get(player).unwrap();
    let map = &gs.ecs.fetch::<Level>().map;

    // TODO remove allow if there is more actions
    #[allow(clippy::single_match)]
    match action {
        TargetingAction::TargetingFromItem(item, range) => {
            if let Some(name) = names.get(item) {
                ctx.print_color(
                    5,
                    0,
                    RGB::named(rltk::YELLOW),
                    RGB::named(rltk::BLACK),
                    "Select Target for ".to_string() + name.name.as_str(),
                );
            }

            let mut available_points = Vec::new();
            for point in view.visible_tiles.iter() {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(player_pos, *point);
                if distance <= range as f32 {
                    ctx.set_bg(point.x, point.y, RGB::named(rltk::BLUE));
                    available_points.push(*point);
                }
            }

            let mouse_pos = rltk::Point::new(ctx.mouse_pos().0, ctx.mouse_pos().1);

            if available_points.contains(&mouse_pos) {
                ctx.set_bg(mouse_pos.x, mouse_pos.y, RGB::named(rltk::PINK));
                if ctx.left_click {
                    gs.targeting_pos = mouse_pos;
                    return TargetingMenuAction::Selected;
                }
            } else {
                ctx.set_bg(mouse_pos.x, mouse_pos.y, RGB::named(rltk::RED));
                if ctx.left_click {
                    return TargetingMenuAction::NoResponse;
                }
            }

            if available_points.contains(&gs.targeting_pos) {
                ctx.set_bg(
                    gs.targeting_pos.x,
                    gs.targeting_pos.y,
                    RGB::named(rltk::ORANGE),
                );
            } else {
                ctx.set_bg(
                    gs.targeting_pos.x,
                    gs.targeting_pos.y,
                    RGB::named(rltk::RED),
                );
            }
        }
        // unimplemented
        _ => (),
    }

    let input = get_input(ctx);

    // TODO remove allow if there is more actions
    #[allow(clippy::collapsible_match)]
    match input {
        None => return TargetingMenuAction::NoResponse,
        Some(key) => match key {
            InputType::Escape => return TargetingMenuAction::Cancel,
            InputType::Down => {
                gs.targeting_pos.y += 1;
            }
            InputType::DownRight => {
                gs.targeting_pos.y += 1;
                gs.targeting_pos.x += 1;
            }
            InputType::DownLeft => {
                gs.targeting_pos.y += 1;
                gs.targeting_pos.x -= 1;
            }
            InputType::Up => {
                gs.targeting_pos.y -= 1;
            }
            InputType::UpLeft => {
                gs.targeting_pos.y -= 1;
                gs.targeting_pos.x -= 1;
            }
            InputType::UpRight => {
                gs.targeting_pos.y -= 1;
                gs.targeting_pos.x += 1;
            }
            InputType::Left => {
                gs.targeting_pos.x -= 1;
            }
            InputType::Right => {
                gs.targeting_pos.x += 1;
            }
            InputType::Enter => {
                return TargetingMenuAction::Selected;
            }
            _ => return TargetingMenuAction::NoResponse,
        },
    }
    gs.targeting_pos.y = gs.targeting_pos.y.max(0).min(map.height_max() as i32);
    gs.targeting_pos.x = gs.targeting_pos.x.max(0).min(map.width_max() as i32);

    TargetingMenuAction::NoResponse
}
