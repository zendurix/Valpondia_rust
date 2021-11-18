use std::collections::HashMap;

use itertools::Itertools;
use rltk::Rltk;
use specs::{Entity, Join, WorldExt};

use crate::{
    ecs::{components, State},
    impl_window_option_selector,
};

use super::menus::{TextCol, WindowOptionSelector};

#[derive(PartialEq, Copy, Clone)]
pub enum InventoryMenuAction {
    Cancel,
    NoResponse,
    SelectedItem(Entity),
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuAction {
    Cancel,
    NoResponse,
    Use(Entity),
    Drop(Entity),
    Equip(Entity),
    UnEquip(Entity),
}
#[derive(Debug, Clone)]
pub struct GuiInventoryManager {
    pub selected: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub bg: rltk::RGB,

    pub title: TextCol,
    pub options: Vec<TextCol>,
    pub options_ent: Vec<Entity>,
}

impl WindowOptionSelector for GuiInventoryManager {
    impl_window_option_selector!();

    fn options(&self) -> &[TextCol] {
        &self.options
    }
}

impl GuiInventoryManager {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> GuiInventoryManager {
        GuiInventoryManager {
            x,
            y,
            width,
            height,
            selected: 0,
            bg: rltk::RGB::named(rltk::BLACK),
            title: TextCol::new(vec![(
                "Inventory".to_string(),
                rltk::RGB::named(rltk::WHITE),
            )]),
            options: vec![],
            options_ent: vec![],
        }
    }

    pub fn reset(&mut self, gs: &State) {
        self.selected = 0;
        let player = *gs.ecs.fetch::<Entity>();
        let names = gs.ecs.read_storage::<components::Name>();
        let inventories = gs.ecs.read_storage::<components::InInventory>();
        let equipped = gs.ecs.read_storage::<components::Equipped>();
        let entities = gs.ecs.entities();

        let mut items_groupped = HashMap::<String, (usize, Entity)>::default();

        for (ent, _in_inv, name) in (&entities, &inventories, &names)
            .join()
            .filter(|item| item.1.owner == player)
        {
            let mut name = name.name.clone();
            if equipped.contains(ent) {
                name += " <EQUIPPED> ";
            }
            if let std::collections::hash_map::Entry::Vacant(e) = items_groupped.entry(name.clone())
            {
                e.insert((1, ent));
            } else {
                items_groupped.get_mut(&name).unwrap().0 += 1;
            }
        }

        self.options = items_groupped
            .iter()
            .sorted_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()))
            .map(|(name, (num, _ent))| {
                let mut text = TextCol::new(vec![(name.clone(), rltk::RGB::named(rltk::WHITE))]);
                if *num > 1 {
                    text += (format!("  x{}", num), rltk::RGB::named(rltk::GREEN));
                }
                text
            })
            .collect_vec();
        self.options_ent = items_groupped
            .iter()
            .sorted_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()))
            .map(|(_name, (_num, ent))| *ent)
            .collect_vec();
    }

    pub fn update(&mut self, ctx: &mut Rltk) -> InventoryMenuAction {
        self.draw(ctx);

        let action = self.handle_input(ctx);
        match action {
            crate::graphics::gui::menus::MenuAction::SelectedIndex(i) => {
                InventoryMenuAction::SelectedItem(self.options_ent[i])
            }
            crate::graphics::gui::menus::MenuAction::NotSelected => InventoryMenuAction::NoResponse,
            crate::graphics::gui::menus::MenuAction::Cancel => InventoryMenuAction::Cancel,
        }
    }
}
#[derive(Debug, Clone)]
pub struct GuiItemActionManager {
    pub selected: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub bg: rltk::RGB,

    pub title: TextCol,
    pub options: Vec<TextCol>,
}

impl WindowOptionSelector for GuiItemActionManager {
    impl_window_option_selector!();

    fn options(&self) -> &[TextCol] {
        &self.options
    }
}

impl GuiItemActionManager {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> GuiItemActionManager {
        GuiItemActionManager {
            x,
            y,
            width,
            height,
            selected: 0,
            bg: rltk::RGB::named(rltk::BLACK),
            title: TextCol::new(vec![(
                "Inventory".to_string(),
                rltk::RGB::named(rltk::WHITE),
            )]),
            options: vec![],
        }
    }

    pub fn reset(&mut self, gs: &State, item: Entity) {
        let usables = gs.ecs.read_storage::<components::Usable>();
        let equipables = gs.ecs.read_storage::<components::Equippable>();
        let equipeds = gs.ecs.read_storage::<components::Equipped>();
        let names = gs.ecs.read_storage::<components::Name>();
        let name = names.get(item).unwrap();
        let can_be_used = usables.get(item).is_some();
        let can_be_equipped = equipables.get(item).is_some();
        let can_be_unequipped = equipeds.get(item).is_some();

        self.title = TextCol::simple("Item: ".to_string() + name.name.as_str());

        self.options.clear();
        self.options.push(TextCol::simple("Drop".to_string()));

        if can_be_used {
            self.options.push(TextCol::simple("Use".to_string()));
        }
        if can_be_equipped && !can_be_unequipped {
            self.options.push(TextCol::simple("Equip".to_string()));
        }
        if can_be_unequipped {
            self.options.push(TextCol::simple("UnEquip".to_string()));
        }
    }

    pub fn update(&mut self, ctx: &mut Rltk, item: Entity) -> ItemMenuAction {
        self.draw(ctx);
        let action = self.handle_input(ctx);
        match action {
            super::menus::MenuAction::SelectedIndex(i) => {
                match self.options[i].strings[0].0.as_str() {
                    "Drop" => ItemMenuAction::Drop(item),
                    "Use" => ItemMenuAction::Use(item),
                    "Equip" => ItemMenuAction::Equip(item),
                    "UnEquip" => ItemMenuAction::UnEquip(item),
                    _ => ItemMenuAction::NoResponse,
                }
            }
            super::menus::MenuAction::NotSelected => ItemMenuAction::NoResponse,
            super::menus::MenuAction::Cancel => ItemMenuAction::Cancel,
        }
    }
}
