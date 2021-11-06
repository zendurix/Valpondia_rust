use std::collections::HashSet;

use specs::prelude::*;

use specs_derive::Component;

use crate::base::Dir;
pub use rltk::{VirtualKeyCode, RGB};

use super::systems::player::InputType;

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

#[derive(Component, Debug, Clone)]
pub struct Renderable {
    pub ascii: u16,
    pub texture: Option<()>, // add textures here
    pub fg: RGB,
    pub bg: RGB,
    /// If more then one entities are on same pos ent with lowest order is drawn.
    /// 0 - player, 1 - monster, 2 - items
    pub render_order: i32,
}

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub input: Option<InputType>,
}

#[derive(Component, Debug, Clone)]
pub struct Movable {
    pub move_dir: Option<Dir>,
}

#[derive(Component, Debug, Clone)]
pub struct AI {}

#[derive(Component, Debug, Clone)]
pub struct View {
    pub range: usize,
    pub visible_tiles: HashSet<rltk::Point>,
    pub should_update: bool,
}

#[derive(Component, Debug, Clone)]
pub struct ViewMemory {
    pub seen_tiles: HashSet<rltk::Point>,
    pub should_update: bool,
}

#[derive(Component, Debug, Clone)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug, Clone)]
pub struct BlocksTile {}

#[derive(Component, Debug, Clone)]
pub struct Hp {
    pub max_hp: i32,
    pub hp: i32,
}

#[derive(Component, Debug, Clone)]
pub struct CombatBaseStats {
    pub attack: i32,
    pub defense: i32,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToMeleeAtack {
    pub target: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Item {}

#[derive(Component, Debug, Clone)]
pub struct Heal {
    pub heal_power: i32,
}

#[derive(Component, Debug, Clone)]
pub struct HealEffect {
    pub heal_power: i32,
}

#[derive(Component, Debug, Clone)]
pub struct InInventory {
    pub owner: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub item: Entity,
}

/// if target is None, then target is user
#[derive(Component, Debug, Clone)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<rltk::Point>,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct Usable {
    pub destoyed_on_use: bool,
}

#[derive(Component, Debug, Clone)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, Debug, Clone)]
pub struct InflictsDamage {
    pub damage: i32,
}

/// AOE effect (circle area for now only)
#[derive(Component, Debug, Clone)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Component, Debug, Clone)]
pub struct Sleeping {
    pub duration: usize,
}

#[derive(Component, Debug, Clone)]
pub struct SleepingEffect {
    pub duration: usize,
}
