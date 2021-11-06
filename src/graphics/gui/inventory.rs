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

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> InventoryMenuAction {
    let player = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<components::Name>();
    let inventories = gs.ecs.read_storage::<components::InInventory>();
    let entities = gs.ecs.entities();

    let inv_count = (&inventories, &names)
        .join()
        .filter(|item| item.0.owner == *player)
        .count();

    let drawer = &mut gs.gui_drawer;

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

    let mut inv_entities = vec![];

    let inventory = (&entities, &inventories, &names)
        .join()
        .filter(|item| item.1.owner == *player);
    for (i, (ent, _inv, name)) in inventory.enumerate() {
        inv_entities.push(ent);
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

        ctx.print(21, y, &name.name.to_string());
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
                    InventoryMenuAction::SelectedItem(inv_entities[key_press_as_inv_index as usize])
                } else {
                    InventoryMenuAction::NoResponse
                }
            }
        },
    }
}
