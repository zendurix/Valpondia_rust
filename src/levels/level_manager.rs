use rltk::Point;

use crate::maps::{
    generators::{
        basic_dungeon::{BasicDungeonMap, BasicDungeonMapConfig},
        cellular_automata::CAMapGen,
        test_map::TestMap,
    },
    genrate_map_and_spawn_areas, MapGenerator,
};

use super::level::{Level, LevelType};

use crate::levels::errors::Result;

///
#[derive(Debug)]
pub struct LevelManager {
    pub current_level_index: usize,
    pub levels: Vec<Level>,
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
        depth: usize,
        prev_down_stairs_pos: Option<Point>,
    ) -> Result<usize> {
        let (map, spawn_areas) = match level_type {
            LevelType::TestLevel => {
                let gen = TestMap::new(width, height);
                (gen.map(), vec![])
            }
            LevelType::Cave => {
                let gen = CAMapGen::new(width, height)?;
                genrate_map_and_spawn_areas(gen, prev_down_stairs_pos)?
            }
            LevelType::BasicDungeon => {
                let gen = BasicDungeonMap::new(width, height, BasicDungeonMapConfig::default());
                genrate_map_and_spawn_areas(gen, prev_down_stairs_pos)?
            }
        };
        let new_level = Level {
            map,
            depth,
            level_index: self.levels.len(),
            level_weight: 1,
            spawn_areas,
        };
        self.levels.push(new_level);
        Ok(self.levels.len() - 1)
    }
}
