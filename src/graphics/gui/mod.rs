use rltk::{Console, Rltk, RGB};
use specs::prelude::*;

use crate::{ecs::components, gamelog::GameLog, maps::Map};

pub const CONSOLE_BOX_HEIGHT: usize = 8;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk, window_width: usize, window_height: usize) {
    // Draw mouse cursor
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));

    draw_console_box(ecs, ctx, window_width, window_height);

    draw_player_health(ecs, ctx, window_width, window_height);

    draw_gamelog(ecs, ctx, window_width, window_height);

    draw_cursor_tooltips(ecs, ctx, window_width, window_height);
}

fn draw_console_box(ecs: &World, ctx: &mut Rltk, window_width: usize, window_height: usize) {
    ctx.draw_box(
        0,
        window_height - (CONSOLE_BOX_HEIGHT + 1),
        window_width - 1,
        CONSOLE_BOX_HEIGHT,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
}

fn draw_player_health(ecs: &World, ctx: &mut Rltk, window_width: usize, window_height: usize) {
    let hps = ecs.read_storage::<components::Hp>();
    let players = ecs.read_storage::<components::Player>();
    for (_player, hp) in (&players, &hps).join() {
        let health = format!(" HP: {} / {} ", hp.hp, hp.max_hp);
        ctx.print_color(
            10,
            window_height - (CONSOLE_BOX_HEIGHT + 1),
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &health,
        );

        ctx.draw_bar_horizontal(
            30,
            window_height - (CONSOLE_BOX_HEIGHT + 1),
            50,
            hp.hp,
            hp.max_hp,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
        );
    }
}

fn draw_gamelog(ecs: &World, ctx: &mut Rltk, window_width: usize, window_height: usize) {
    let log = ecs.fetch::<GameLog>();

    let mut y = window_height - (CONSOLE_BOX_HEIGHT);
    for s in log.entries.iter().rev() {
        if y < window_height - 1 {
            ctx.print(2, y, s);
        }
        y += 1;
    }
}

fn draw_cursor_tooltips(ecs: &World, ctx: &mut Rltk, window_width: usize, window_height: usize) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<components::Name>();
    let positions = ecs.read_storage::<components::Position>();

    let player = *ecs.read_resource::<Entity>();
    let views = ecs.read_storage::<components::View>();
    let view = views.get(player).unwrap();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width as i32 || mouse_pos.1 >= map.height as i32 {
        return;
    }

    let mut tooltip_msg: Vec<String> = Vec::new();
    for (name, pos) in (&names, &positions).join() {
        if pos.x as i32 == mouse_pos.0
            && pos.y as i32 == mouse_pos.1
            && view
                .visible_tiles
                .contains(&rltk::Point::new(pos.x as i32, pos.y as i32))
        {
            tooltip_msg.push(name.name.to_string());
        }
    }

    if !tooltip_msg.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip_msg.iter() {
            if width < s.len() as i32 {
                width = s.len() as i32;
            }
        }
        width += 3;

        if mouse_pos.0 > window_width as i32 / 2 {
            let arrow_pos = rltk::Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip_msg.iter() {
                ctx.print_color(
                    left_x,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::GREY),
                    s,
                );
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x - i,
                        y,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::GREY),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                &"->".to_string(),
            );
        } else {
            let arrow_pos = rltk::Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y = mouse_pos.1;
            for s in tooltip_msg.iter() {
                ctx.print_color(
                    left_x + 1,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::GREY),
                    s,
                );
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x + 1 + i,
                        y,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::GREY),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                &"<-".to_string(),
            );
        }
    }
}
