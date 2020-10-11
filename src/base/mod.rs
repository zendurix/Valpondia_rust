use specs::prelude::*;
use specs_derive::Component;

pub use rltk::{VirtualKeyCode as Key, RGB};

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Center,
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    LevelDown,
    LevelUp,
}
