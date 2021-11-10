pub mod corridor;
pub(crate) mod errors;
pub mod generators;
pub mod map;
pub mod rect;

pub use errors::Error;

use errors::Result;
use rltk::Point;

pub use self::map::{Map, TileType};

pub trait MapGenerator {
    fn generate(&mut self, prev_down_stairs_pos: Option<Point>) -> Result<()>;
    fn map(self) -> Map;
}

pub trait SpawnAreas {
    fn spawn_areas(&self) -> Vec<Vec<(usize, usize)>>;
}

#[allow(clippy::type_complexity)]
pub fn genrate_map_and_spawn_areas<T: MapGenerator + SpawnAreas>(
    mut generator: T,
    prev_down_stairs_pos: Option<Point>,
) -> Result<(Map, Vec<Vec<(usize, usize)>>)> {
    generator.generate(prev_down_stairs_pos)?;
    let spawn_areas = generator.spawn_areas();
    Ok((generator.map(), spawn_areas))
}
