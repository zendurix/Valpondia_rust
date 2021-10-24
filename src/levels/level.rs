use super::map::Map;


#[derive(Debug, Clone, Copy)]
pub enum LevelType {
    Cave,
    Dungeon
}


pub struct Level {
    map: Map,
    level_index: usize,
}
