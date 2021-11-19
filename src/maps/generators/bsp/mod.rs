
mod tree;


use rltk::Point;

use crate::{maps::{Map, rect::Rect}, rng};

use self::tree::BTree;

use super::{MapGenerator, common::{apply_horizontal_tunnel, apply_vertical_tunnel}};

use crate::
    maps::
        errors::Result;



pub struct BSPConfig {
    pub rooms_min: usize,
    pub rooms_max: usize,
    pub room_size_min: usize,
    pub room_size_max: usize,
}

impl Default for BSPConfig {
    fn default() -> BSPConfig {
        BSPConfig {
            rooms_min: 6,
            rooms_max: 9,
            room_size_min: 8,
            room_size_max: 15,
        }
    }
}

pub struct BSPMap {
    width: usize,
    height: usize,
    config: BSPConfig,

    tree: BTree,

    
    
    map: Map,
    rooms: Vec<Rect>,

    #[cfg(feature = "map_gen_testing")]
    history: Vec<Map>,
}

impl BSPMap {
    pub fn new(width: usize, height: usize, config: BSPConfig) -> BSPMap {
        BSPMap {
            width,
            height,
            config,
            map: Map::new(width, height).with_all_solid(),
            rooms: vec![],

            #[cfg(feature = "map_gen_testing")]
            history: vec![],
        }
    }

}







impl MapGenerator for BSPMap {
    fn generate(&mut self, prev_down_stairs_pos: Option<Point>) -> Result<()> {
        

        Ok(())
    }

    fn reset(&mut self) {
        #[cfg(feature = "map_gen_testing")]
        self.history.clear();
        self.map = Map::new(self.width, self.height).with_all_solid();
    }

    fn map(&self) -> Map {
        self.map.clone()
    }

    fn spawn_areas(&self) -> Vec<Vec<(usize, usize)>> {
        self.rooms.iter().map(|r| r.area_within()).collect()
    }

    #[cfg(feature = "map_gen_testing")]
    fn history(&self) -> Vec<Map> {
        self.history.clone()
    }
}
