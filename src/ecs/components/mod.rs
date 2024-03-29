use std::collections::{HashMap, HashSet};

use specs::prelude::*;

use specs_derive::Component;

use crate::{graphics::gui::menus::TextCol, spawner::spawn_tables::SpawnEntry};
pub use rltk::{VirtualKeyCode, RGB};

use super::systems::player::{Dir, InputType};

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

#[derive(Component, Debug, Clone)]
pub struct Renderable {
    pub ascii: u16,
    pub texture: Option<usize>, // texture index
    pub fg: RGB,
    pub bg: RGB,
    /// If more then one entities are on same pos, entity with lowest order is drawn.
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
    /// indexed by level
    pub seen_tiles: HashMap<usize, HashSet<rltk::Point>>,
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
pub struct Inventory {
    pub items: Vec<Entity>,
}

impl Inventory {
    pub fn new_empty() -> Inventory {
        Inventory { items: vec![] }
    }
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

#[derive(Component, Debug, Clone)]
pub struct Teleporting {}

#[derive(Component, Debug, Clone)]
pub struct TeleportingEffect {
    pub target_pos: (usize, usize),
}

#[derive(Component, Debug, Clone)]
pub struct SpawnsAfterDeath {
    pub spawns: Vec<SpawnEntry>,
}

#[derive(Component, Debug, Clone)]
pub struct Spawn {
    pub names_nums: Vec<(String, usize)>,
}

#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash)]
pub enum BodyPart {
    /// manly used for weapons (don't use as actual body part in `BodyParts`)
    OneHanded,
    /// manly used for weapons (don't use as actual body part in `BodyParts`)
    TwoHanded,

    HandRight,
    HandLeft,

    Head,
    Body,
    Hands,
    Feet,
}

impl ToString for BodyPart {
    fn to_string(&self) -> String {
        match self {
            BodyPart::OneHanded => "One Handed".to_string(),
            BodyPart::TwoHanded => "Two Handed".to_string(),
            BodyPart::HandRight => "Right Hand".to_string(),
            BodyPart::HandLeft => "Left  Hand".to_string(),
            BodyPart::Head => "Head".to_string(),
            BodyPart::Body => "Body".to_string(),
            BodyPart::Hands => "Hands".to_string(),
            BodyPart::Feet => "Feet".to_string(),
        }
    }
}

impl BodyPart {
    pub fn is_hand(self) -> bool {
        self == BodyPart::HandLeft || self == BodyPart::HandRight
    }
}

#[derive(Component, Debug, Clone)]
pub struct BodyParts {
    pub parts_with_equipped: HashMap<BodyPart, Option<Entity>>,
}

impl BodyParts {
    /// default body parts for standard humanoid (2 hands, head, body, Feet)
    pub fn default_humanoid() -> BodyParts {
        let mut parts_with_equipped = HashMap::default();
        parts_with_equipped.insert(BodyPart::HandRight, None);
        parts_with_equipped.insert(BodyPart::HandLeft, None);
        parts_with_equipped.insert(BodyPart::Head, None);
        parts_with_equipped.insert(BodyPart::Body, None);
        parts_with_equipped.insert(BodyPart::Hands, None);
        parts_with_equipped.insert(BodyPart::Feet, None);
        BodyParts {
            parts_with_equipped,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Equippable {
    pub body_part: BodyPart,
}

#[derive(Component, Debug, Clone)]
pub struct Equipped {
    pub owner: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToEquip {
    pub item: Entity,
    pub target_body_part: BodyPart,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToUnEquip {
    pub item: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct MeleeDamageBonus {
    pub power: i32,
}

#[derive(Component, Clone, Debug)]
pub struct DefenseBonus {
    pub defense: i32,
}

#[derive(Component, Clone, Debug)]
pub struct Description {
    pub title: TextCol,
    pub text: TextCol,
}

#[derive(Component, Clone, Debug)]
pub struct FinalBoss {}
