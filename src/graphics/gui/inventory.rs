use std::collections::HashMap;

use itertools::Itertools;
use rltk::{Rltk, RGB};
use specs::{Entity, Join, WorldExt};

use crate::ecs::{
    components,
    systems::player::{input::get_input, InputType},
    State,
};

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
}

pub struct GuiInventoryManager {
    pub selected_option_index: usize,
}

impl GuiInventoryManager {
    pub fn show_inventory(&mut self, gs: &mut State, ctx: &mut Rltk) -> InventoryMenuAction {
        let player = *gs.ecs.fetch::<Entity>();
        let names = gs.ecs.read_storage::<components::Name>();
        let inventories = gs.ecs.read_storage::<components::InInventory>();
        let entities = gs.ecs.entities();

        let mut inv_entities = vec![];

        let mut items_groupped = HashMap::<String, (usize, Entity)>::default();

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

        let inv_count = items_groupped.len();

        let _drawer = &mut gs.gui_drawer;

        let mut y = (25 - (inv_count / 2)) as i32;
        ctx.draw_box(
            15,
            y - 2,
            31,
            (inv_count + 3) as i32,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );
        ctx.print_color(
            18,
            y - 2,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            "Inventory",
        );
        ctx.print_color(
            18,
            y + inv_count as i32 + 1,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            "press ESCAPE to exit",
        );

        for (i, (name, (count, first_entity))) in items_groupped
            .into_iter()
            .sorted_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()))
            .enumerate()
        {
            inv_entities.push(first_entity);
            ctx.set(
                17,
                y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('('),
            );
            ctx.set(
                18,
                y,
                RGB::named(rltk::YELLOW),
                RGB::named(rltk::BLACK),
                97 + i as rltk::FontCharType,
            );
            ctx.set(
                19,
                y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437(')'),
            );

            let name_len = name.len();
            ctx.print(21, y, name);
            ctx.print_color(
                23 + name_len,
                y,
                RGB::named(rltk::GREEN),
                RGB::named(rltk::BLACK),
                format!("x{}", count),
            );
            y += 1;
        }

        let input = get_input(ctx);

        match input {
            None => InventoryMenuAction::NoResponse,
            Some(key) => match key {
                InputType::Escape => InventoryMenuAction::Cancel,
                _ => {
                    let key_press_as_inv_index = rltk::letter_to_option(ctx.key.unwrap());

                    if (0..inv_count as i32).contains(&key_press_as_inv_index) {
                        InventoryMenuAction::SelectedItem(
                            inv_entities[key_press_as_inv_index as usize],
                        )
                    } else {
                        InventoryMenuAction::NoResponse
                    }
                }
            },
        }
    }

    pub fn show_item_actions(
        &mut self,
        gs: &mut State,
        ctx: &mut Rltk,
        item: Entity,
    ) -> ItemMenuAction {
        let usables = gs.ecs.read_storage::<components::Usable>();
        let names = gs.ecs.read_storage::<components::Name>();
        let name = names.get(item).unwrap();
        let can_be_used = usables.get(item).is_some();

        ctx.draw_box(
            15,
            10,
            31,
            5,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );
        ctx.print_color(
            18,
            10,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            "Item: ".to_string() + name.name.as_str(),
        );

        ctx.set(
            17,
            11,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            11,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('d'),
        );
        ctx.print_color(
            19,
            11,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            ") - drop",
        );

        if can_be_used {
            ctx.set(
                17,
                12,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('('),
            );
            ctx.set(
                18,
                12,
                RGB::named(rltk::GREEN),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('u'),
            );
            ctx.print_color(
                19,
                12,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                ") - use",
            );
        }

        ctx.print_color(
            18,
            14,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            "press ESCAPE to exit",
        );

        let input = get_input(ctx);
        match input {
            None => ItemMenuAction::NoResponse,
            Some(key) => match key {
                InputType::Escape => ItemMenuAction::Cancel,
                InputType::U => {
                    if can_be_used {
                        ItemMenuAction::Use(item)
                    } else {
                        ItemMenuAction::NoResponse
                    }
                }
                InputType::D => ItemMenuAction::Drop(item),
                _ => ItemMenuAction::NoResponse,
            },
        }
    }
}
