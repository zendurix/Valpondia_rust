use crate::maps::Map;

#[derive(Debug, Clone, Copy)]
pub enum LevelType {
    Cave,
    BasicDungeon,
    TestLevel,
}

#[derive(Debug)]
pub struct Level {
    pub map: Map,
    pub level_index: usize,
}
