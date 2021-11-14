use rltk::Rltk;
use specs::{Entity, WorldExt};

use crate::{
    ecs::{components, State},
    impl_window_option_selector,
};

use super::menus::{TextCol, WindowOptionSelector};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum EquipmentMenuAction {
    NoResponse,
    Cancel,
    //...
}

#[derive(Debug, Clone)]
pub struct GuiEquipmentManager {
    pub selected: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub bg: rltk::RGB,

    pub title: TextCol,
    pub options: Vec<TextCol>,
}

impl WindowOptionSelector for GuiEquipmentManager {
    impl_window_option_selector!();

    fn options(&self) -> &[TextCol] {
        &self.options
    }
}

impl GuiEquipmentManager {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> GuiEquipmentManager {
        GuiEquipmentManager {
            x,
            y,
            width,
            height,
            selected: 0,
            bg: rltk::RGB::named(rltk::BLACK),
            title: TextCol::new(vec![(
                "Equipment".to_string(),
                rltk::RGB::named(rltk::WHITE),
            )]),
            options: vec![],
        }
    }

    pub fn reset(&mut self, gs: &State) {
        let _equipables = gs.ecs.read_storage::<components::Equippable>();
        let equiments = gs.ecs.read_storage::<components::BodyParts>();
        let player = *gs.ecs.fetch::<Entity>();
        let equipment = equiments.get(player).unwrap();
        let names = gs.ecs.read_storage::<components::Name>();

        let mut slots = vec![];
        for (body_part, item_equipped) in equipment.parts_with_equipped.iter() {
            let mut str = TextCol::new(vec![(
                body_part.to_string() + ":  ",
                rltk::RGB::named(rltk::GREY),
            )]);
            if let Some(item) = item_equipped {
                let item_name = names.get(*item).unwrap();
                str += (item_name.name.clone(), rltk::RGB::named(rltk::WHITE));
            } else {
                str += ("Nothing".to_string(), rltk::RGB::named(rltk::GREY));
            }
            slots.push(str);
        }
        self.options.clear();
        self.options = slots;
    }

    pub fn update(&mut self, ctx: &mut Rltk) -> EquipmentMenuAction {
        self.draw(ctx);
        let action = self.handle_input(ctx);
        match action {
            super::menus::MenuAction::SelectedIndex(_i) => {
                // TODO
                EquipmentMenuAction::NoResponse
            }
            super::menus::MenuAction::NotSelected => EquipmentMenuAction::NoResponse,
            super::menus::MenuAction::Cancel => EquipmentMenuAction::Cancel,
        }
    }
}
