use std::default;

use rltk::RandomNumberGenerator;

use crate::data::map::{Map, TileType};

use super::MapGenerator;

pub struct CAMapGenConfig {
    pub alive_on_start_chance_percent: usize,
    pub step_limit: usize,
    /// if less neighbours than dies
    pub death_limit: usize,
    /// if more neighbours than alive
    pub birth_limit: usize,
    pub min_cave_size_percent: usize,
}

impl Default for CAMapGenConfig {
    fn default() -> CAMapGenConfig {
        CAMapGenConfig {
            alive_on_start_chance_percent: 45,
            step_limit: 8,
            death_limit: 3,
            birth_limit: 4,
            min_cave_size_percent: 50,
        }
    }
}

#[derive(Debug, Clone)]
struct CAPlace {
    alive: bool,
    checked: bool,
    cave_index: usize,
    x: usize,
    y: usize,
}

impl Default for CAPlace {
    fn default() -> CAPlace {
        CAPlace {
            alive: false,
            checked: false,
            cave_index: 0,
            x: 0,
            y: 0,
        }
    }
}

pub struct CAMapGen {
    ca_map: Vec<CAPlace>,
    width: usize,
    height: usize,
    config: CAMapGenConfig,
}

impl CAMapGen {
    pub fn new(width: usize, height: usize) -> CAMapGen {
        let mut ca_map = vec![CAPlace::default(); width * height];
        let mut x = 0;
        let mut y = 0;
        for place in ca_map.iter_mut() {
            place.x = x;
            place.y = y;
            x += 1;
            if x > width - 1 {
                y += 1;
                x = 0;
            }
        }

        CAMapGen {
            ca_map,
            width,
            height,
            config: CAMapGenConfig::default(),
        }
    }

    pub fn with_config(mut self, config: CAMapGenConfig) -> CAMapGen {
        self.config = config;
        self
    }

    pub fn make_cave_map(&mut self) -> Map {
        self.set_random_state();

        for _ in 0..=self.config.step_limit {
            self.make_step();
        }

        let mut map = Map::new(self.width, self.height);
        self.set_map(&mut map);
        map.with_edges_solid()
    }

    fn set_map(&self, map: &mut Map) {
        for (i, place) in self.ca_map.iter().enumerate() {
            let tile_type = if place.alive {
                TileType::Wall
            } else {
                TileType::Floor
            };
            map.set_tile_at_index(i, tile_type);
        }
    }

    fn set_random_state(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        for place in self.ca_map.iter_mut() {
            place.alive = rng.range(0, 101) <= self.config.alive_on_start_chance_percent;
        }
    }

    fn make_step(&mut self) {
        let current_map_state = self.ca_map.clone();
        for place in self.ca_map.iter_mut() {
            let neighbours_count = Self::count_neighbours(
                &current_map_state,
                place,
                self.width as i32,
                self.height as i32,
            );
            if place.alive {
                place.alive = neighbours_count >= self.config.death_limit;
            } else {
                place.alive = neighbours_count > self.config.birth_limit;
            }
        }
    }

    fn count_neighbours(
        current_map_state: &[CAPlace],
        place: &CAPlace,
        width: i32,
        height: i32,
    ) -> usize {
        let mut count = 0;
        if Self::is_alive_on_xy(
            current_map_state,
            place.x as i32 - 1,
            place.y as i32,
            width,
            height,
        ) {
            count += 1;
        }
        if Self::is_alive_on_xy(
            current_map_state,
            place.x as i32 + 1,
            place.y as i32,
            width,
            height,
        ) {
            count += 1;
        }
        if Self::is_alive_on_xy(
            current_map_state,
            place.x as i32,
            place.y as i32 - 1,
            width,
            height,
        ) {
            count += 1;
        }
        if Self::is_alive_on_xy(
            current_map_state,
            place.x as i32,
            place.y as i32 + 1,
            width,
            height,
        ) {
            count += 1;
        }
        if Self::is_alive_on_xy(
            current_map_state,
            place.x as i32 - 1,
            place.y as i32 - 1,
            width,
            height,
        ) {
            count += 1;
        }
        if Self::is_alive_on_xy(
            current_map_state,
            place.x as i32 + 1,
            place.y as i32 - 1,
            width,
            height,
        ) {
            count += 1;
        }
        if Self::is_alive_on_xy(
            current_map_state,
            place.x as i32 - 1,
            place.y as i32 + 1,
            width,
            height,
        ) {
            count += 1;
        }
        if Self::is_alive_on_xy(
            current_map_state,
            place.x as i32 + 1,
            place.y as i32 + 1,
            width,
            height,
        ) {
            count += 1;
        }
        return count;
    }

    fn is_alive_on_xy(
        current_map_state: &[CAPlace],
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> bool {
        if x < 0 || x >= width || y < 0 || y >= height {
            false
        } else {
            current_map_state[Self::xy_to_index(x as usize, y as usize, width as usize)].alive
        }
    }

    fn xy_to_index(x: usize, y: usize, width: usize) -> usize {
        x + y * width
    }
}

impl MapGenerator for CAMapGen {
    fn generate(mut self) -> Map {
        self.make_cave_map()
    }
}
