use specs::prelude::*;
use specs_derive::Component;

use crate::base::{Dir, Key, RGB};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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
    pub input: Option<Key>,
}

#[derive(Component)]
pub struct Movable {
    pub move_dir: Option<Dir>,
}

#[derive(Component)]
pub struct AI {
}
