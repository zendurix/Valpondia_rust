use rltk::Point;

use crate::{
    maps::{errors::Result, Map, TileType},
    rng,
};

use super::MapGenerator;

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

    fn add_up_and_down_stairs(&mut self, prev_down_stairs_pos: Option<Point>) {
        let mut random_point = 0;
        if let Some(prev_stairs) = prev_down_stairs_pos {
            let index = self
                .map
                .xy_to_index(prev_stairs.x as usize, prev_stairs.y as usize);

            if !self.map.tiles[index].blocks_movement() {
                self.map.tiles[index] = TileType::StairsUp;
            }
        }

        while self.map.tiles[random_point].blocks_movement()
            || self.map.tiles[random_point] == TileType::StairsUp
        {
            random_point = rng::range(
                self.width as i32 + 1,
                (self.width * (self.height - 1)) as i32,
            ) as usize;
        }
        self.map.tiles[random_point] = TileType::StairsDown;
    }
}

impl MapGenerator for TestMap {
    fn generate(&mut self, prev_down_stairs_pos: Option<Point>) -> Result<()> {
        self.map = self.map.clone().with_edges_solid();
        self.add_up_and_down_stairs(prev_down_stairs_pos);
        Ok(())
    }

    fn reset(&mut self) {
        self.map = Map::new(self.width, self.height).with_edges_solid();
    }

    fn map(&self) -> Map {
        self.map.clone()
    }

    fn spawn_areas(&self) -> Vec<Vec<(usize, usize)>> {
        vec![vec![(30, 30)]]
    }

    #[cfg(feature = "map_gen_testing")]
    fn history(&self) -> Vec<(Map, String)> {
        vec![]
    }
}
