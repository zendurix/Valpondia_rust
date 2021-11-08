use rltk::Point;

use crate::maps::{errors::Result, Map, MapGenerator};

pub struct TestMap {
    width: usize,
    height: usize,
}

impl TestMap {
    pub fn new(width: usize, height: usize) -> TestMap {
        TestMap { width, height }
    }
}

impl MapGenerator for TestMap {
    fn generate(self, _prev_down_stairs_pos: Option<Point>) -> Result<Map> {
        let map = Map::new(self.width, self.height);
        Ok(map.with_edges_solid())
    }
}
