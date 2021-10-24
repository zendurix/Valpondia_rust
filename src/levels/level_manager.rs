use super::level::{Level, LevelType};

///
pub struct LevelManager {
    pub current_level_index: usize,
    levels: Vec<Level>,
}

impl LevelManager {
    pub fn new() -> LevelManager {
        LevelManager {
            current_level_index: 0,
            levels: vec![],
        }
    }

    pub fn current_level(&self) -> &Level {
        &self.levels[self.current_level_index]
    }

    pub fn current_level_mut(&mut self) -> &mut Level {
        &mut self.levels[self.current_level_index]
    }


    /// returns new level index
    pub fn crete_new_level(&mut self, level_type: LevelType, width: usize, height: usize) -> Result<usize> {

    }
}
