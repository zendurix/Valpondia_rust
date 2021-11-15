use itertools::Itertools;
use specs::prelude::*;

use crate::{
    ecs::components::{self, BodyPart},
    gamelog::GameLog,
};

pub struct ItemEquipSystem {}

impl<'a> System<'a> for ItemEquipSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, components::WantsToEquip>,
        WriteStorage<'a, components::Equippable>,
        ReadStorage<'a, components::Name>,
        WriteStorage<'a, components::Equipped>,
        WriteStorage<'a, components::BodyParts>,
        ReadStorage<'a, components::Inventory>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            player,
            entities,
            mut gamelog,
            mut wants_eq,
            equipables,
            names,
            mut equippeds,
            mut eqs,
            invs,
        ) = data;

        let mut items_to_unequip = vec![];
        for (ent, want_eq, inv, eq) in (&entities, &mut wants_eq, &invs, &mut eqs).join() {
            let item = want_eq.item;

            if inv.items.contains(&item) && equipables.contains(item) {
                let body_part = want_eq.target_body_part;

                if body_part == BodyPart::TwoHanded {
                    if eq.parts_with_equipped.contains_key(&BodyPart::HandLeft)
                        && eq.parts_with_equipped.contains_key(&BodyPart::HandRight)
                    {
                        let slot_l = eq.parts_with_equipped.get_mut(&BodyPart::HandLeft).unwrap();
                        if let Some(prev_equipped) = slot_l {
                            items_to_unequip.push((ent, *prev_equipped));
                        }
                        *slot_l = Some(item);

                        let slot_r = eq
                            .parts_with_equipped
                            .get_mut(&BodyPart::HandRight)
                            .unwrap();
                        if let Some(prev_equipped) = slot_r {
                            items_to_unequip.push((ent, *prev_equipped));
                        }
                        *slot_r = Some(item);

                        equippeds
                            .insert(item, components::Equipped { owner: ent })
                            .expect("Cannot insert equipped");
                    }
                } else {
                    if let Some(slot) = eq.parts_with_equipped.get_mut(&body_part) {
                        if let Some(prev_equipped) = slot {
                            items_to_unequip.push((ent, *prev_equipped));
                        }
                        *slot = Some(item);
                        equippeds
                            .insert(item, components::Equipped { owner: ent })
                            .expect("Cannot insert equipped");
                    }
                }
            }

            if ent == *player {
                gamelog.entries.push(format!(
                    "You equip up the {}.",
                    names.get(item).unwrap().name
                ));
            }
        }

        for (owner, unequip) in items_to_unequip {
            unequip_item(&mut eqs, &mut equippeds, owner, unequip);
        }

        wants_eq.clear();
    }
}

pub fn insert_item_in_eq(ecs: &mut World, owner: Entity, item: Entity) {
    let in_inv = ecs.write_storage::<components::InInventory>();
    let mut eqs = ecs.write_storage::<components::BodyParts>();
    let mut equippeds = ecs.write_storage::<components::Equipped>();
    let equipables = ecs.write_storage::<components::Equippable>();

    let eq = eqs.get_mut(owner).unwrap();

    let mut items_to_unequip = vec![];

    if in_inv.contains(item) && equipables.contains(item) {
        let body_part = equipables.get(item).unwrap().body_part;

        if body_part == BodyPart::TwoHanded {
            if eq.parts_with_equipped.contains_key(&BodyPart::HandLeft)
                && eq.parts_with_equipped.contains_key(&BodyPart::HandRight)
            {
                let slot_l = eq.parts_with_equipped.get_mut(&BodyPart::HandLeft).unwrap();
                if let Some(prev_equipped) = slot_l {
                    items_to_unequip.push(*prev_equipped);
                }
                *slot_l = Some(item);

                let slot_r = eq
                    .parts_with_equipped
                    .get_mut(&BodyPart::HandRight)
                    .unwrap();
                if let Some(prev_equipped) = slot_r {
                    items_to_unequip.push(*prev_equipped);
                }
                *slot_r = Some(item);

                equippeds
                    .insert(item, components::Equipped { owner })
                    .expect("Cannot insert equipped");
            }
        } else if body_part == BodyPart::OneHanded {
            let mut target_hand = BodyPart::HandRight;

            if let Some(slot_r) = eq.parts_with_equipped.get_mut(&BodyPart::HandRight) {
                if slot_r.is_none() {
                    target_hand = BodyPart::HandRight;
                } else if let Some(slot_l) = eq.parts_with_equipped.get_mut(&BodyPart::HandLeft) {
                    if slot_l.is_none() {
                        target_hand = BodyPart::HandLeft;
                    } else {
                        target_hand = BodyPart::HandRight;
                    }
                }
            } else if let Some(_slot_l) = eq.parts_with_equipped.get_mut(&BodyPart::HandLeft) {
                target_hand = BodyPart::HandLeft;
            }

            let slot = eq.parts_with_equipped.get_mut(&target_hand).unwrap();
            if let Some(prev_equipped) = slot {
                items_to_unequip.push(*prev_equipped);
            }
            *slot = Some(item);

            equippeds
                .insert(item, components::Equipped { owner })
                .expect("Cannot insert equipped");
        } else {
            if let Some(slot) = eq.parts_with_equipped.get_mut(&body_part) {
                if let Some(prev_equipped) = slot {
                    items_to_unequip.push(*prev_equipped);
                }
                *slot = Some(item);
                equippeds
                    .insert(item, components::Equipped { owner })
                    .expect("Cannot insert equipped");
            }
        }
    }

    for unequip in items_to_unequip {
        unequip_item(&mut eqs, &mut equippeds, owner, unequip);
    }
}

pub struct ItemUnEquipSystem {}

impl<'a> System<'a> for ItemUnEquipSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, components::WantsToUnEquip>,
        ReadStorage<'a, components::Name>,
        WriteStorage<'a, components::Equipped>,
        WriteStorage<'a, components::BodyParts>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            player,
            entities,
            mut gamelog,
            mut wants_uneq,
            names,
            mut equippeds,
            mut eqs,
        ) = data;

        for (ent, want_uneq, eq) in (&entities, &mut wants_uneq, &mut eqs).join() {
            let item = want_uneq.item;

            let body_parts = eq
                .parts_with_equipped
                .iter()
                .filter(|(_part, item_equipped)| {
                    item_equipped.is_some() && item_equipped.unwrap() == item
                })
                .map(|(part, _item)| *part)
                .collect_vec();

            for part in body_parts {
                *eq.parts_with_equipped.get_mut(&part).unwrap() = None;
            }
            equippeds.remove(item);

            if ent == *player {
                gamelog.entries.push(format!(
                    "You unequip the {}.",
                    names.get(item).unwrap().name
                ));
            }
        }
        wants_uneq.clear();
    }
}

pub fn unequip_item(
    eqs: &mut WriteStorage<components::BodyParts>,
    equippeds: &mut WriteStorage<components::Equipped>,
    owner: Entity,
    item: Entity,
) {
    let eq = eqs.get_mut(owner).unwrap();

    let body_parts = eq
        .parts_with_equipped
        .iter()
        .filter(|(_part, item_equipped)| item_equipped.is_some() && item_equipped.unwrap() == item)
        .map(|(part, _item)| *part)
        .collect_vec();

    for part in body_parts {
        *eq.parts_with_equipped.get_mut(&part).unwrap() = None;
    }
    equippeds.remove(item);
}
