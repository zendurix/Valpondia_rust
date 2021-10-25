use crate::maps::Map;

#[derive(Debug, Clone, Copy)]
pub enum LevelType {
    Cave,
    Dungeon,
}

pub struct Level {
    pub map: Map,
    pub level_index: usize,
}
