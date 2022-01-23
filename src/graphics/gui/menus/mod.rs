use rltk::{Rltk, RGB};

use crate::{
    ecs::systems::player::{input::get_input, InputType},
    graphics::window::{
        SPRITE_16x16_CONSOLE_INDEX, SPRITE_32x32_CONSOLE_INDEX, CHAR_CONSOLE_INDEX,
    },
};

use super::inventory::INV_TEXT_COL;

pub mod main_menu;

#[cfg(feature = "map_gen_testing")]
pub mod map_testing;

pub enum MenuAction {
    SelectedIndex(usize),
    NotSelected,
    Cancel,
}

pub type StrCol = (String, RGB);

#[derive(Debug, Clone)]
pub struct TextCol {
    pub strings: Vec<StrCol>,
    pub bg: rltk::RGB,
}

impl TextCol {
    pub fn empty() -> TextCol {
        TextCol {
            strings: vec![],
            bg: rltk::RGB::named(rltk::BLACK),
        }
    }

    ///  white string with black background
    pub fn simple(string: String) -> TextCol {
        TextCol {
            strings: vec![(string, rltk::RGB::named(INV_TEXT_COL))],
            bg: rltk::RGB::named(rltk::BLACK),
        }
    }

    /// with default black background
    pub fn new(strings: Vec<StrCol>) -> TextCol {
        assert!(!strings.is_empty());
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
        self.strings.iter().map(|(s, _c)| s.len()).sum()
    }

    pub fn print(&self, ctx: &mut Rltk, x: usize, y: usize) {
        let mut current_x = x;
        for (s, c) in self.strings.iter() {
            ctx.print_color(current_x, y, *c, self.bg, s);
            current_x += s.len()
        }
    }
}

impl std::ops::AddAssign<TextCol> for TextCol {
    fn add_assign(&mut self, other: TextCol) {
        self.strings.extend(other.strings.into_iter());
    }
}

impl std::ops::AddAssign<StrCol> for TextCol {
    fn add_assign(&mut self, str_col: StrCol) {
        self.strings.push(str_col);
    }
}

pub trait WindowOptionSelector {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn bg(&self) -> rltk::RGB;

    fn title(&self) -> &TextCol;
    fn options(&self) -> &[TextCol];
    fn options_sprites_indexes(&self) -> &[Option<usize>];
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
        ctx.set_active_console(SPRITE_32x32_CONSOLE_INDEX);
        ctx.draw_box(
            self.x() / 2 - 1,
            self.y() / 2 - 1,
            self.width() / 2 + 1,
            self.height() / 2 + 1,
            RGB::named(rltk::BLACK),
            RGB::named(rltk::BLACK),
        );

        ctx.set_active_console(CHAR_CONSOLE_INDEX);
        ctx.draw_box_double(
            self.x(),
            self.y(),
            self.width(),
            self.height(),
            RGB::named(INV_TEXT_COL),
            RGB::named(rltk::BLACK),
        );

        self.title().print(ctx, self.x() + 2, self.y());

        let mut current_y = self.y() + 1;

        for (i, opt) in self
            .options()
            .iter()
            // .sorted_by(|a, b| {
            //     a.strings[0]
            //         .0
            //         .to_lowercase()
            //         .cmp(&b.strings[0].0.to_lowercase())
            // })
            .enumerate()
        {
            let bg = if self.selected() == i {
                let mut bg = self.bg();
                bg.r += 0.5;
                bg.g += 0.5;
                bg.b += 0.5;
                bg
            } else {
                self.bg()
            };

            ctx.set(
                self.x() + 1,
                current_y,
                RGB::named(INV_TEXT_COL),
                bg,
                rltk::to_cp437('('),
            );
            ctx.set(
                self.x() + 2,
                current_y,
                if self.selected() == i {
                    RGB::named(rltk::GREEN)
                } else {
                    RGB::named(INV_TEXT_COL)
                },
                bg,
                97 + i as rltk::FontCharType,
            );
            ctx.print_color(self.x() + 3, current_y, RGB::named(INV_TEXT_COL), bg, ") ");

            // if i < self.options_sprites_indexes().len() {
            //     if let Some(sprite_index) = self.options_sprites_indexes()[i] {
            //         draw_batch.set(
            //             rltk::Point::new(self.x() as i32 + 5, current_y as i32),
            //             rltk::ColorPair::new(
            //                 RGB::from_f32(1.0, 1., 1.0),
            //                 RGB::from_f32(0., 0., 0.),
            //             ),
            //             sprite_index,
            //         );
            //     }
            // }

            let opt_selected = opt.clone().with_bg(bg);
            opt_selected.print(ctx, self.x() + 6, current_y);

            current_y += 1;
        }

        ctx.print_color(
            self.x() + 2,
            self.y() + self.height(),
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            "press ESCAPE to exit",
        );
    }

    fn handle_input(&mut self, ctx: &mut Rltk) -> MenuAction {
        let input = get_input(ctx);
        if let Some(key) = input {
            match key {
                InputType::Escape => MenuAction::Cancel,
                InputType::Up => {
                    self.decr_selection();
                    MenuAction::NotSelected
                }
                InputType::Down => {
                    self.incr_selection();
                    MenuAction::NotSelected
                }
                InputType::Enter => MenuAction::SelectedIndex(self.selected()),
                _ => {
                    let key_press_as_inv_index = rltk::letter_to_option(ctx.key.unwrap());

                    if (0..self.options().len() as i32).contains(&key_press_as_inv_index) {
                        MenuAction::SelectedIndex(key_press_as_inv_index as usize)
                    } else {
                        MenuAction::NotSelected
                    }
                }
            }
        } else {
            MenuAction::NotSelected
        }
    }
}

#[macro_export]
macro_rules! impl_window_option_selector {
    () => {
        fn x(&self) -> usize {
            self.x
        }

        fn y(&self) -> usize {
            self.y
        }

        fn width(&self) -> usize {
            self.width
        }

        fn height(&self) -> usize {
            self.height
        }

        fn bg(&self) -> rltk::RGB {
            self.bg
        }

        fn title(&self) -> &TextCol {
            &self.title
        }

        fn selected(&self) -> usize {
            self.selected
        }

        fn selected_mut(&mut self) -> &mut usize {
            &mut self.selected
        }

        fn options_sprites_indexes(&self) -> &[Option<usize>] {
            &self.options_sprites_indexes
        }
    };
}
