extern crate rltk;
extern crate specs;
extern crate specs_derive;

use specs::prelude::*;
use specs_derive::Component;

use rltk::RGB;

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
