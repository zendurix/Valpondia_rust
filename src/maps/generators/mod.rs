use rltk::Point;

use crate::rng;

use self::{
    basic_dungeon::{BasicDungeonMap, BasicDungeonMapConfig},
    cellular_automata::CAMapGen,
    test_map::TestMap,
};

use super::{errors::Result, Map};

// use super::MapGenerator;

pub mod basic_dungeon;
pub mod cellular_automata;
mod common;
pub mod test_map;

pub fn random_map_generator(width: usize, height: usize) -> Box<dyn MapGenerator> {
    let rand = rng::range(0, 1);

    match rand {
        1 => Box::new(BasicDungeonMap::new(
            width,
            height,
            BasicDungeonMapConfig::default(),
        )),
        2 => Box::new(CAMapGen::new(width, height).unwrap()),

        _ => Box::new(TestMap::new(width, height)),
    }
}

pub trait MapGenerator {
    fn generate(&mut self, prev_down_stairs_pos: Option<Point>) -> Result<()>;
    fn map(self) -> Map;

    fn spawn_areas(&self) -> Vec<Vec<(usize, usize)>>;
}

#[allow(clippy::type_complexity)]
pub fn genrate_map_and_spawn_areas<T: MapGenerator>(
    mut generator: T,
    prev_down_stairs_pos: Option<Point>,
) -> Result<(Map, Vec<Vec<(usize, usize)>>)> {
    generator.generate(prev_down_stairs_pos)?;
    let spawn_areas = generator.spawn_areas();
    Ok((generator.map(), spawn_areas))
}
