use rltk::Point;

use crate::maps::{errors::Result, Map, MapGenerator};

pub struct TestMap {
    pub width: usize,
    pub height: usize,
    map: Map,
}

impl TestMap {
    pub fn new(width: usize, height: usize) -> TestMap {
        TestMap {
            width,
            height,
            map: Map::new(width, height),
        }
    }
}

impl MapGenerator for TestMap {
    fn generate(&mut self, _prev_down_stairs_pos: Option<Point>) -> Result<()> {
        self.map = self.map.clone().with_edges_solid();
        Ok(())
    }
    fn map(self) -> Map {
        self.map
    }
}
