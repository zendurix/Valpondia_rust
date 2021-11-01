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
    pub level: i16,
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
pub struct OccupiesTile {}