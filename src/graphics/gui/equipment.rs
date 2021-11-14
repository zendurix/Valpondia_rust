use specs::{Entity, WorldExt};

use crate::{
    ecs::{components, State},
    graphics::gui::equipment,
    impl_window_option_selector,
};

use super::menus::{TextCol, WindowOptionSelector};

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
        let equipables = gs.ecs.read_storage::<components::Equippable>();
        let equiments = gs.ecs.read_storage::<components::BodyParts>();
        let player = *gs.ecs.fetch::<Entity>();
        let equipment = equiments.get(player).unwrap();
        let names = gs.ecs.read_storage::<components::Name>();

        let mut slots = vec![];
        for (body_part, item_equipped) in equipment.parts_with_equipped {
            let mut str = body_part.to_string();
            if let Some(item) = item_equipped {
                let item_name = names.get(item).unwrap();
                str += item_name.name.as_str();
            }
            slots.push(str);
        }

        for (ent, _in_inv, name) in (&entities, &inventories, &names)
            .join()
            .filter(|item| item.1.owner == player)
        {
            if items_groupped.contains_key(&name.name) {
                items_groupped.get_mut(&name.name).unwrap().0 += 1;
            } else {
                items_groupped.insert(name.name.clone(), (1, ent));
            }
        }

        let name = names.get(item).unwrap();
        let can_be_used = usables.get(item).is_some();
        let can_be_equipped = equipables.get(item).is_some();

        self.title = TextCol::simple("Item: ".to_string() + name.name.as_str());

        self.options.clear();
        self.options.push(TextCol::simple("Drop".to_string()));

        if can_be_used {
            self.options.push(TextCol::simple("Use".to_string()));
        }
        if can_be_equipped {
            self.options.push(TextCol::simple("Equip".to_string()));
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
                    _ => ItemMenuAction::NoResponse,
                }
            }
            super::menus::MenuAction::NotSelected => ItemMenuAction::NoResponse,
            super::menus::MenuAction::Cancel => ItemMenuAction::Cancel,
        }
    }
}
