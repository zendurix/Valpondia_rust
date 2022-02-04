use rltk::Point;

use crate::{
    maps::{errors::Result, Map, TileType},
    rng,
};

use super::MapGenerator;

pub struct DrunkardWalkConfig {
    min_area_perc: usize,
    drunkard_life: usize,
}

impl Default for DrunkardWalkConfig {
    fn default() -> DrunkardWalkConfig {
        DrunkardWalkConfig {
            min_area_perc: 50,
            drunkard_life: 400,
        }
    }
}

pub struct DrunkardWalkGen {
    width: usize,
    height: usize,
    config: DrunkardWalkConfig,
    map: Map,

    #[cfg(feature = "map_gen_testing")]
    history: Vec<(Map, String)>,
}

impl DrunkardWalkGen {
    pub fn new(width: usize, height: usize, config: DrunkardWalkConfig) -> DrunkardWalkGen {
        DrunkardWalkGen {
            width,
            height,
            config,
            map: Map::new(width, height).with_all_solid(),

            #[cfg(feature = "map_gen_testing")]
            history: vec![],
        }
    }

    pub fn create_drunkard_walk_map(&mut self, prev_down_stairs_pos: Option<Point>) {
        #[cfg(feature = "map_gen_testing")]
        self.history.push((self.map.clone(), "Start".to_string()));

        let start_pos = if let Some(prev_stairs) = prev_down_stairs_pos {
            prev_stairs
        } else {
            Point::new(self.width as i32 / 2, self.height as i32 / 2)
        };

        let centr_idx = self
            .map
            .xy_to_index(start_pos.x as usize, start_pos.y as usize);
        self.map.tiles[centr_idx] = TileType::Floor;

        let mut floor_tile_perc = self.map.floor_tiles_perc();

        // let mut digger_count = 0;
        // let mut active_digger_count = 0;

        while floor_tile_perc < self.config.min_area_perc {
            let mut did_something = false;
            let mut drunk_x = start_pos.x as usize;
            let mut drunk_y = start_pos.y as usize;
            let mut drunkard_life = self.config.drunkard_life;

            while drunkard_life > 0 {
                let drunk_idx = self.map.xy_to_index(drunk_x, drunk_y);
                if self.map.tiles[drunk_idx] == TileType::Wall {
                    did_something = true;
                }

                #[cfg(feature = "map_gen_testing")]
                {
                    self.map.tiles[drunk_idx] = TileType::TestWall;
                }
                #[cfg(not(feature = "map_gen_testing"))]
                {
                    self.map.tiles[drunk_idx] = TileType::Floor;
                }

                let stagger_direction = rng::roll_dice(1, 4);
                match stagger_direction {
                    1 => {
                        if drunk_x > 2 {
                            drunk_x -= 1;
                        }
                    }
                    2 => {
                        if drunk_x < self.map.width - 2 {
                            drunk_x += 1;
                        }
                    }
                    3 => {
                        if drunk_y > 2 {
                            drunk_y -= 1;
                        }
                    }
                    _ => {
                        if drunk_y < self.map.height - 2 {
                            drunk_y += 1;
                        }
                    }
                }

                drunkard_life -= 1;
            }
            if did_something {
                #[cfg(feature = "map_gen_testing")]
                {
                    self.history.push((self.map.clone(), "TODO".to_string()));
                }
            }
            // active_digger_count += 1;

            //digger_count += 1;

            #[cfg(feature = "map_gen_testing")]
            {
                for t in self.map.tiles.iter_mut() {
                    if *t == TileType::TestWall {
                        *t = TileType::Floor;
                    }
                }
            }
            floor_tile_perc = self.map.floor_tiles_perc();
        }
    }

    fn add_up_and_down_stairs(&mut self, prev_down_stairs_pos: Option<Point>) {
        if let Some(prev_stairs) = prev_down_stairs_pos {
            let index = self
                .map
                .xy_to_index(prev_stairs.x as usize, prev_stairs.y as usize);

            if !self.map.tiles[index].blocks_movement() {
                self.map.tiles[index] = TileType::StairsUp;
            }
        }

        let mut random_point = 0;

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

    fn replace_debug_walls_with_walls(&mut self) {
        for t in self.map.tiles.iter_mut() {
            if *t == TileType::TestWall {
                *t = TileType::Wall;
            }
        }
    }
}

impl MapGenerator for DrunkardWalkGen {
    fn generate(&mut self, prev_down_stairs_pos: Option<Point>) -> Result<()> {
        self.create_drunkard_walk_map(prev_down_stairs_pos);
        self.add_up_and_down_stairs(prev_down_stairs_pos);
        self.replace_debug_walls_with_walls();
        #[cfg(feature = "map_gen_testing")]
        {
            self.history
                .push((self.map.clone(), "Finished".to_string()));
        }
        Ok(())
    }

    fn reset(&mut self) {
        #[cfg(feature = "map_gen_testing")]
        {
            self.history.clear();
        }
        self.map = Map::new(self.width, self.height).with_all_solid();
    }

    fn map(&self) -> Map {
        self.map.clone()
    }

    fn spawn_areas(&self) -> Vec<Vec<(usize, usize)>> {
        let area = self.area();
        let areas_num = area.len() / 50;
        let mut areas = vec![];
        for _ in 0..areas_num {
            areas.push(area.clone());
        }
        areas
    }

    #[cfg(feature = "map_gen_testing")]
    fn history(&self) -> Vec<(Map, String)> {
        self.history.clone()
    }
}
