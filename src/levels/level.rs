use crate::maps::Map;

#[derive(Debug, Clone, Copy)]
pub enum LevelType {
    Cave,
    BasicDungeon,
    TestLevel,
}

#[derive(Debug, Clone)]
pub struct Level {
    pub map: Map,
    pub level_index: usize,
    pub depth: usize,
}
