use crate::maps::{
    generators::{
        basic_dungeon::{BasicDungeonMap, BasicDungeonMapConfig},
        cellular_automata::CAMapGen,
        test_map::TestMap,
    },
    MapGenerator,
};

use super::level::{Level, LevelType};

use crate::levels::errors::Result;

///
#[derive(Debug)]
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
    pub fn crete_new_level(
        &mut self,
        level_type: LevelType,
        width: usize,
        height: usize,
    ) -> Result<usize> {
        match level_type {
            LevelType::TestLevel => {
                let new_map = TestMap::new(width, height).generate()?;
                let new_level = Level {
                    map: new_map,
                    level_index: self.levels.len(),
                };
                self.levels.push(new_level);
            }
            LevelType::Cave => {
                let new_map = CAMapGen::new(width, height)?.generate()?;
                let new_level = Level {
                    map: new_map,
                    level_index: self.levels.len(),
                };
                self.levels.push(new_level);
            }
            LevelType::BasicDungeon => {
                let new_map = BasicDungeonMap::new(width, height, BasicDungeonMapConfig::default())
                    .generate()?;
                let new_level = Level {
                    map: new_map,
                    level_index: self.levels.len(),
                };
                self.levels.push(new_level);
            }
        }
        Ok(self.levels.len() - 1)
    }
}
