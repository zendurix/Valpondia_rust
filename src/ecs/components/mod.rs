use std::collections::HashSet;

use specs::prelude::*;

use specs::storage::NullStorage;
use specs_derive::Component;

use crate::base::Dir;
pub use rltk::{VirtualKeyCode, RGB};

use super::systems::player::InputType;

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

#[derive(Component)]
pub struct Renderable {
    pub ascii: u16,
    pub texture: Option<()>, // add textures here
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Player {
    pub input: Option<InputType>,
}

#[derive(Component)]
pub struct Movable {
    pub move_dir: Option<Dir>,
}

#[derive(Default)]
pub struct AI;
impl Component for AI {
    type Storage = NullStorage<AI>;
}

#[derive(Component)]
pub struct View {
    pub range: usize,
    pub visible_tiles: HashSet<rltk::Point>,
    pub should_update: bool,
}

#[derive(Component)]
pub struct ViewMemory {
    pub seen_tiles: HashSet<rltk::Point>,
    pub should_update: bool,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}
#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct Hp {
    pub max_hp: i32,
    pub hp: i32,
}

#[derive(Component, Debug)]
pub struct CombatBaseStats {
    pub attack: i32,
    pub defense: i32,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToMeleeAtack {
    pub target: Entity,
}

#[derive(Component, Debug)]
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

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct Heal {
    pub heal_power: i32,
}

#[derive(Component, Debug, Clone)]
pub struct InInventory {
    pub owner: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub who: Entity,
    pub item: Entity,
}

#[derive(Component, Debug)]
pub struct WantsToUseItem {
    pub item: Entity,
}
