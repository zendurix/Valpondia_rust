use rltk::Point;

use crate::maps::generators::{
    basic_dungeon::{BasicDungeonMap, BasicDungeonMapConfig},
    bsp::{interior::BSPInteriorGen, BSPConfig, BSPDungeonGen},
    cellular_automata::CAMapGen,
    drunkard_walk::{DrunkardWalkConfig, DrunkardWalkGen},
    genrate_map_and_spawn_areas,
    test_map::TestMap,
    MapGenerator,
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

    pub fn reset(&mut self) {
        self.levels.clear();
        self.current_level_index = 0;
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
                let mut gen = TestMap::new(width, height);
                gen.generate(prev_down_stairs_pos)?;
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
            LevelType::BSPDungeon => {
                let gen = BSPDungeonGen::new(width, height, BSPConfig::default());
                genrate_map_and_spawn_areas(gen, prev_down_stairs_pos)?
            }
            LevelType::BSPInterior => {
                let gen = BSPInteriorGen::new(width, height, BSPConfig::default());
                genrate_map_and_spawn_areas(gen, prev_down_stairs_pos)?
            }
            LevelType::DrunkardWalk => {
                let gen = DrunkardWalkGen::new(width, height, DrunkardWalkConfig::default());
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
