use crate::maps::Map;

#[derive(Debug, Clone, Copy)]
pub enum LevelType {
    Cave,
    BasicDungeon,
    BSPDungeon,
    BSPInterior,
    DrunkardWalk,
    TestLevel,
}

#[derive(Debug, Clone)]
pub struct Level {
    pub map: Map,
    pub level_index: usize,
    pub depth: usize,

    // used for spawn tables
    pub level_weight: usize,

    pub spawn_areas: Vec<Vec<(usize, usize)>>,
}
