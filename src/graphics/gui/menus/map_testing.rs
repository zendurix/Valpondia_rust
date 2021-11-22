#![cfg(feature = "map_gen_testing")]

use rltk::Rltk;

use crate::{
    impl_window_option_selector,
    maps::generators::{test_map::TestMap, MapGenerator},
};

use super::{MenuAction, TextCol, WindowOptionSelector};

#[derive(PartialEq, Copy, Clone)]
pub enum MapGenTestingMenuAction {
    SwitchShowSteps,
    TestBasicDungeonGenerator,
    TestCaMapGen,
    TestBSPDungeonGen,
    TestBSPInteriorGen,
    TestDrunkardWalkGen,

    //... more to come!
    Cancel,
    NoResponse,
}

impl From<usize> for MapGenTestingMenuAction {
    fn from(v: usize) -> MapGenTestingMenuAction {
        match v {
            0 => MapGenTestingMenuAction::SwitchShowSteps,
            1 => MapGenTestingMenuAction::TestBasicDungeonGenerator,
            2 => MapGenTestingMenuAction::TestCaMapGen,
            3 => MapGenTestingMenuAction::TestBSPDungeonGen,
            4 => MapGenTestingMenuAction::TestBSPInteriorGen,
            5 => MapGenTestingMenuAction::TestDrunkardWalkGen,

            _ => MapGenTestingMenuAction::NoResponse,
        }
    }
}

pub struct GuiMapGenTestingManager {
    pub selected: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub bg: rltk::RGB,
    pub show_steps: bool,

    pub title: TextCol,
    pub options: Vec<TextCol>,

    pub map_gen: Box<dyn MapGenerator>,
    pub current_history_index: usize,
}

impl WindowOptionSelector for GuiMapGenTestingManager {
    impl_window_option_selector!();

    fn options(&self) -> &[TextCol] {
        &self.options
    }
}

impl GuiMapGenTestingManager {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> GuiMapGenTestingManager {
        GuiMapGenTestingManager {
            x,
            y,
            width,
            height,
            selected: 0,
            bg: rltk::RGB::named(rltk::BLACK),
            title: TextCol::new(vec![(
                "Map generators testing".to_string(),
                rltk::RGB::named(rltk::WHITE),
            )]),
            options: vec![],
            show_steps: false,
            map_gen: Box::new(TestMap::new(width - 4, height - 4)),
            current_history_index: 0,
        }
    }

    pub fn reset_map_gen(&mut self, mut gen: Box<dyn MapGenerator>) {
        gen.generate(None).unwrap();
        self.map_gen = gen;
        self.current_history_index = 0;
    }

    pub fn reset_current_map_gen(&mut self) {
        self.map_gen.reset();
        self.map_gen.generate(None).unwrap();
        self.current_history_index = 0;
    }

    pub fn reset(&mut self) {
        self.selected = 0;
        //self.show_steps = false;
        self.options = vec![
            TextCol::new(vec![
                ("Show steps:  ".to_string(), rltk::RGB::named(rltk::WHITE)),
                (
                    format!("{}", self.show_steps),
                    if self.show_steps {
                        rltk::RGB::named(rltk::GREEN)
                    } else {
                        rltk::RGB::named(rltk::ORANGE)
                    },
                ),
            ]),
            TextCol::new(vec![(
                "Test Basic Dungeon Generator".to_string(),
                rltk::RGB::named(rltk::WHITE),
            )]),
            TextCol::new(vec![(
                "Test Cellular Automata Cave Generator".to_string(),
                rltk::RGB::named(rltk::WHITE),
            )]),
            TextCol::new(vec![(
                "Test Binary Space Partitioning dungeons Generator".to_string(),
                rltk::RGB::named(rltk::WHITE),
            )]),
            TextCol::new(vec![(
                "Test Binary Space Partitioning Interior Generator".to_string(),
                rltk::RGB::named(rltk::WHITE),
            )]),
            TextCol::new(vec![(
                "Test Drunkard walk map generator".to_string(),
                rltk::RGB::named(rltk::WHITE),
            )]),
        ]
    }

    pub fn update(&mut self, ctx: &mut Rltk) -> MapGenTestingMenuAction {
        self.draw(ctx);

        let action = self.handle_input(ctx);
        match action {
            MenuAction::SelectedIndex(i) => MapGenTestingMenuAction::from(i),
            MenuAction::NotSelected => MapGenTestingMenuAction::NoResponse,
            MenuAction::Cancel => MapGenTestingMenuAction::Cancel,
        }
    }

    pub fn switch_show_steps(&mut self) {
        self.show_steps = !self.show_steps;
    }
}
