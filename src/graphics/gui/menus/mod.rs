use itertools::Itertools;
use rltk::{Rltk, RGB};

use crate::ecs::systems::player::input::get_input;

pub mod main_menu;

pub type StrCol = (String, RGB);

pub struct TextCol {
    pub strings: Vec<StrCol>,
    pub bg: rltk::RGB,
}

impl TextCol {
    /// with default black background
    pub fn new(strings: Vec<StrCol>) -> TextCol {
        assert!(strings.len() > 0);
        TextCol {
            strings,
            bg: rltk::RGB::named(rltk::BLACK),
        }
    }

    pub fn with_bg(mut self, bg: rltk::RGB) -> TextCol {
        self.bg = bg;
        self
    }

    pub fn chars_len(&self) -> usize {
        self.strings.iter().map(|(s, c)| s.len()).sum()
    }

    pub fn print(&self, ctx: &mut Rltk, x: usize, y: usize) {
        let mut current_x = x;
        for (s, c) in self.strings.iter() {
            ctx.print_color(current_x, y, *c, self.bg, s);
            current_x += s.len()
        }
    }
}

pub trait WindowOptionSelector {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn bg(&self) -> rltk::RGB;

    fn title(&self) -> TextCol;
    fn options(&self) -> Vec<TextCol>;
    fn selected(&self) -> usize;
    fn selected_mut(&mut self) -> &mut usize;

    fn incr_selection(&mut self) {
        if self.selected() < self.options().len() - 1 {
            *self.selected_mut() += 1;
        } else {
            *self.selected_mut() = 0;
        }
    }

    fn decr_selection(&mut self) {
        if self.selected() > 0 {
            *self.selected_mut() -= 1;
        } else {
            *self.selected_mut() = self.options().len() - 1;
        }
    }

    /// TODO add result
    fn draw(&self, ctx: &mut Rltk) {
        self.title().print(ctx, self.x() + 2, self.y());

        let mut current_y = self.y() + 1;

        for (i, opt) in self
            .options()
            .iter()
            .sorted_by(|a, b| {
                a.strings[0]
                    .0
                    .to_lowercase()
                    .cmp(&b.strings[0].0.to_lowercase())
            })
            .enumerate()
        {
            ctx.set(
                self.x() + 1,
                current_y,
                RGB::named(rltk::WHITE),
                self.bg(),
                rltk::to_cp437('('),
            );
            ctx.set(
                self.x() + 2,
                current_y,
                if self.selected() == i {
                    RGB::named(rltk::GREEN)
                } else {
                    RGB::named(rltk::WHITE)
                },
                self.bg(),
                97 + i as rltk::FontCharType,
            );
            ctx.set(
                self.x() + 3,
                current_y,
                RGB::named(rltk::WHITE),
                self.bg(),
                rltk::to_cp437(')'),
            );

            opt.print(ctx, self.x() + 5, current_y)
        }
    }

    fn handle_input(&mut self, ctx: &mut Rltk) {
        
    }
}
