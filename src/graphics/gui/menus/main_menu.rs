use rltk::{Rltk, RGB};

use crate::{
    ecs::systems::player::{input::get_input, InputType},
    graphics::gui::{MainMenuAction, MainMenuSelection},
};

pub struct MainMenu {
    title: String,
    options: Vec<(String, MainMenuSelection)>,
    selected: u8,
    title_y: usize,
    first_opt_y: usize,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            title: "Rust roguelike".to_string(),
            options: vec![
                ("New Game".to_string(), MainMenuSelection::NewGame),
                ("Load Game".to_string(), MainMenuSelection::LoadGame),
                ("Credits".to_string(), MainMenuSelection::Credits),
                ("Quit".to_string(), MainMenuSelection::Quit),
            ],
            selected: 0,
            title_y: 15,
            first_opt_y: 20,
        }
    }

    pub fn incr_selection(&mut self) {
        if (self.selected as usize) < self.options.len() - 1 {
            self.selected += 1;
        } else {
            self.selected = 0;
        }
    }

    pub fn decr_selection(&mut self) {
        if (self.selected as usize) > 0 {
            self.selected -= 1;
        } else {
            self.selected = (self.options.len() - 1) as u8;
        }
    }

    pub fn draw(&mut self, ctx: &mut Rltk) -> MainMenuAction {
        ctx.print_color_centered(
            self.title_y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            self.title.clone(),
        );

        for (opt, i) in self.options.iter() {
            let color = if *i as u8 == self.selected {
                RGB::named(rltk::GREEN)
            } else {
                RGB::named(rltk::WHITE)
            };

            ctx.print_color_centered(
                self.first_opt_y + (*i as usize),
                color,
                RGB::named(rltk::BLACK),
                opt,
            );
        }

        let input = get_input(ctx);
        if let Some(key) = input {
            match key {
                InputType::Escape => MainMenuAction::Selected(MainMenuSelection::Quit),
                InputType::Up => {
                    self.decr_selection();
                    MainMenuAction::NotSelected
                }
                InputType::Down => {
                    self.incr_selection();
                    MainMenuAction::NotSelected
                }
                InputType::Enter => {
                    let selection = self.options[self.selected as usize].1;
                    MainMenuAction::Selected(selection)
                }
                _ => MainMenuAction::NotSelected,
            }
        } else {
            MainMenuAction::NotSelected
        }
    }
}
