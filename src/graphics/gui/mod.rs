mod inventory;
mod targeting;

use rltk::{Rltk, RGB};
use specs::prelude::*;

use crate::{ecs::components, gamelog::GameLog, maps::Map};

pub use inventory::{show_inventory, show_item_actions, InventoryMenuAction, ItemMenuAction};
pub use targeting::{show_targeting, TargetingMenuAction};

pub struct GuiDrawer {
    pub window_width: usize,
    pub window_height: usize,
    pub console_box_height: usize,
}

impl GuiDrawer {
    pub fn draw_ui(&self, ecs: &World, ctx: &mut Rltk) {
        // Draw mouse cursor
        let mouse_pos = ctx.mouse_pos();
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));

        self.draw_console_box(ctx);

        self.draw_player_health(ecs, ctx);

        self.draw_gamelog(ecs, ctx);

        self.draw_cursor_tooltips(ecs, ctx);
    }

    fn draw_console_box(&self, ctx: &mut Rltk) {
        ctx.draw_box(
            0,
            self.window_height - (self.console_box_height + 1),
            self.window_width - 1,
            self.console_box_height,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );
    }

    fn draw_player_health(&self, ecs: &World, ctx: &mut Rltk) {
        let hps = ecs.read_storage::<components::Hp>();
        let players = ecs.read_storage::<components::Player>();
        for (_player, hp) in (&players, &hps).join() {
            let health = format!(" HP: {} / {} ", hp.hp, hp.max_hp);
            ctx.print_color(
                10,
                self.window_height - (self.console_box_height + 1),
                RGB::named(rltk::YELLOW),
                RGB::named(rltk::BLACK),
                &health,
            );

            ctx.draw_bar_horizontal(
                30,
                self.window_height - (self.console_box_height + 1),
                50,
                hp.hp,
                hp.max_hp,
                RGB::named(rltk::RED),
                RGB::named(rltk::BLACK),
            );
        }
    }

    fn draw_gamelog(&self, ecs: &World, ctx: &mut Rltk) {
        let log = ecs.fetch::<GameLog>();

        let mut y = self.window_height - (self.console_box_height);
        for s in log.entries.iter().rev() {
            if y < self.window_height - 1 {
                ctx.print(2, y, s);
            }
            y += 1;
        }
    }

    fn draw_cursor_tooltips(&self, ecs: &World, ctx: &mut Rltk) {
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

            if mouse_pos.0 > self.window_width as i32 / 2 {
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
}
