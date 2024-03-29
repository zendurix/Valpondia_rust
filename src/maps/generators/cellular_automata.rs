use std::ops::Not;

use rltk::Point;

use crate::maps::errors::{Error, Result};
use crate::maps::{Map, TileType};
use crate::rng;

use super::MapGenerator;

pub struct CAMapGenConfig {
    pub alive_on_start_chance_percent: usize,
    pub step_limit: usize,
    /// if less neighbours than dies
    pub death_limit: usize,
    /// if more neighbours than alive
    pub birth_limit: usize,
    pub min_cave_size_percent: usize,

    /// only biggest cave will remain
    pub delete_small_caves: bool,
}

impl Default for CAMapGenConfig {
    fn default() -> CAMapGenConfig {
        CAMapGenConfig {
            alive_on_start_chance_percent: 45,
            step_limit: 8,
            death_limit: 3,
            birth_limit: 4,
            min_cave_size_percent: 50,
            delete_small_caves: true,
        }
    }
}

#[derive(Debug, Clone)]
struct CAPlace {
    alive: bool,
    checked: bool,
    cave_index: Option<usize>,
    x: usize,
    y: usize,
}

impl Default for CAPlace {
    fn default() -> CAPlace {
        CAPlace {
            alive: false,
            checked: false,
            cave_index: None,
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
    map: Map,

    #[cfg(feature = "map_gen_testing")]
    history: Vec<(Map, String)>,
}

impl CAMapGen {
    pub fn new(width: usize, height: usize) -> Result<CAMapGen> {
        if width == 0 || height == 0 {
            return Err(Error::IncorrectMapDimensions {
                map_dimensions: (width, height),
            });
        }
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

        Ok(CAMapGen {
            ca_map,
            width,
            height,
            config: CAMapGenConfig::default(),
            map: Map::new(width, height),

            #[cfg(feature = "map_gen_testing")]
            history: vec![],
        })
    }

    pub fn with_config(mut self, config: CAMapGenConfig) -> CAMapGen {
        self.config = config;
        self
    }

    pub fn make_cave_map(&mut self) -> Result<()> {
        #[cfg(feature = "map_gen_testing")]
        self.history.push((self.map.clone(), "Start".to_string()));

        self.set_random_state();

        #[cfg(feature = "map_gen_testing")]
        self.set_map();
        #[cfg(feature = "map_gen_testing")]
        self.history
            .push((self.map.clone(), "Random State".to_string()));

        for i in 0..=self.config.step_limit {
            self.make_step();

            #[cfg(feature = "map_gen_testing")]
            self.set_map();
            #[cfg(feature = "map_gen_testing")]
            self.history.push((
                self.map.clone(),
                format!("Step {} of Cellular Autamata.", i),
            ));
        }

        if self.config.delete_small_caves {
            while !self.delete_small_caves() {
                self.set_random_state();

                for _ in 0..=self.config.step_limit {
                    self.make_step();
                }
            }
        }
        self.set_map();

        #[cfg(feature = "map_gen_testing")]
        self.history
            .push((self.map.clone(), "Finished Map".to_string()));
        Ok(())
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

    fn set_map(&mut self) {
        for (i, place) in self.ca_map.iter().enumerate() {
            let tile_type = if place.alive {
                TileType::Floor
            } else {
                TileType::Rock
            };
            self.map.set_tile_at_index(i, tile_type);
        }
        self.map = self.map.clone().with_edges_solid();
    }

    fn set_random_state(&mut self) {
        for place in self.ca_map.iter_mut() {
            place.alive = rng::range(0, 100) as usize <= self.config.alive_on_start_chance_percent;
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
        count
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

    fn delete_small_caves(&mut self) -> bool {
        let mut cave_surfaces = vec![];

        let places_len = self.ca_map.len();
        for i in 0..places_len {
            if self.ca_map[i].alive && !self.ca_map[i].checked {
                let cave_index = cave_surfaces.len();
                let place_x = self.ca_map[i].x;
                let place_y = self.ca_map[i].y;

                cave_surfaces.push(self.count_cave_surface(place_x, place_y, cave_index));
            }
        }
        if cave_surfaces.is_empty() {
            println!("Reload cave gen");
            return false;
        }

        let max_surface = *cave_surfaces.iter().max().unwrap();
        let max_surface_cave_index = cave_surfaces
            .iter()
            .enumerate()
            .find(|(_i, surface)| **surface == max_surface)
            .unwrap()
            .0;

        for place in self.ca_map.iter_mut() {
            if place
                .cave_index
                .and_then(|ind| (ind != max_surface_cave_index).then(|| ()))
                .is_some()
            {
                place.alive = false;
            }
        }
        true
    }

    fn count_cave_surface(&mut self, place_x: usize, place_y: usize, cave_index: usize) -> usize {
        let mut places_in_cave_postitions = vec![(place_x, place_y)];

        let origin_index = Self::xy_to_index(place_x, place_y, self.width);
        self.ca_map[origin_index].checked = true;
        self.ca_map[origin_index].cave_index = Some(cave_index);

        loop {
            let surface = places_in_cave_postitions.len();
            for i in 0..surface {
                let pos = places_in_cave_postitions[i];
                if let Some(index) = Self::find_next_place_index_in_cave_for_pos(
                    &self.ca_map,
                    pos.0 as i32,
                    pos.1 as i32,
                    self.width as i32,
                    self.height as i32,
                ) {
                    self.ca_map[index].checked = true;
                    self.ca_map[index].cave_index = Some(cave_index);
                    places_in_cave_postitions.push((self.ca_map[index].x, self.ca_map[index].y));
                }
            }

            if places_in_cave_postitions.iter().all(|pos| {
                Self::find_next_place_index_in_cave_for_pos(
                    &self.ca_map,
                    pos.0 as i32,
                    pos.1 as i32,
                    self.width as i32,
                    self.height as i32,
                )
                .is_none()
            }) {
                break;
            }
        }
        places_in_cave_postitions.len()
    }

    fn find_next_place_index_in_cave_for_pos(
        current_map_state: &[CAPlace],
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Option<usize> {
        let try_place_on_xy = |place_x, place_y| -> Option<usize> {
            if Self::is_alive_on_xy(current_map_state, place_x, place_y, width, height) {
                let place_index =
                    Self::xy_to_index(place_x as usize, place_y as usize, width as usize);
                current_map_state[place_index]
                    .checked
                    .not()
                    .then(|| place_index)
            } else {
                None
            }
        };

        try_place_on_xy(x - 1, y)
            .or_else(|| try_place_on_xy(x + 1, y))
            .or_else(|| try_place_on_xy(x, y - 1))
            .or_else(|| try_place_on_xy(x, y + 1))
            .or_else(|| try_place_on_xy(x - 1, y - 1))
            .or_else(|| try_place_on_xy(x + 1, y - 1))
            .or_else(|| try_place_on_xy(x - 1, y + 1))
    }
}

impl MapGenerator for CAMapGen {
    fn generate(&mut self, prev_down_stairs_pos: Option<Point>) -> Result<()> {
        self.make_cave_map()?;
        if let Some(prev_stairs) = prev_down_stairs_pos {
            let index = self
                .map
                .xy_to_index(prev_stairs.x as usize, prev_stairs.y as usize);
            while self.map.tiles[index] != TileType::Floor
                || self.area().len() < self.config.min_cave_size_percent
            {
                self.reset();
                self.make_cave_map()?;
            }
        }
        self.add_up_and_down_stairs(prev_down_stairs_pos);
        Ok(())
    }

    fn reset(&mut self) {
        #[cfg(feature = "map_gen_testing")]
        self.history.clear();
        let mut ca_map = vec![CAPlace::default(); self.width * self.height];
        let mut x = 0;
        let mut y = 0;
        for place in ca_map.iter_mut() {
            place.x = x;
            place.y = y;
            x += 1;
            if x > self.width - 1 {
                y += 1;
                x = 0;
            }
        }
        self.ca_map = ca_map;
    }

    fn map(&self) -> Map {
        self.map.clone()
    }

    fn spawn_areas(&self) -> Vec<Vec<(usize, usize)>> {
        let area = self.area();
        let areas_num = area.len() / 60;
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
