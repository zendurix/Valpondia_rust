use itertools::Itertools;
use rltk::Point;

use crate::{
    maps::{
        errors::Result,
        rect::{apply_room_to_map, Rect},
        Map, TileType,
    },
    rng,
};

use super::{
    common::{apply_horizontal_tunnel, apply_vertical_tunnel},
    MapGenerator,
};

pub struct BasicDungeonMapConfig {
    pub rooms_min: usize,
    pub rooms_max: usize,
    pub room_size_min: usize,
    pub room_size_max: usize,
}

impl Default for BasicDungeonMapConfig {
    fn default() -> BasicDungeonMapConfig {
        BasicDungeonMapConfig {
            rooms_min: 6,
            rooms_max: 9,
            room_size_min: 8,
            room_size_max: 15,
        }
    }
}

pub struct BasicDungeonMap {
    width: usize,
    height: usize,
    config: BasicDungeonMapConfig,
    map: Map,
    rooms: Vec<Rect>,

    #[cfg(feature = "map_gen_testing")]
    history: Vec<(Map, String)>,
}

impl BasicDungeonMap {
    pub fn new(width: usize, height: usize, config: BasicDungeonMapConfig) -> BasicDungeonMap {
        BasicDungeonMap {
            width,
            height,
            config,
            map: Map::new(width, height).with_all_solid(),
            rooms: vec![],

            #[cfg(feature = "map_gen_testing")]
            history: vec![],
        }
    }

    pub fn create_basic_dungeon_map(&mut self, prev_down_stairs_pos: Option<Point>) {
        #[cfg(feature = "map_gen_testing")]
        self.history.push((self.map.clone(), "Start".to_string()));

        self.add_rooms();

        if let Some(prev_stairs) = prev_down_stairs_pos {
            let index = self
                .map
                .xy_to_index(prev_stairs.x as usize, prev_stairs.y as usize);
            while self.map.tiles[index] != TileType::Floor {
                self.map = Map::new(self.width, self.height).with_all_solid();
                self.add_rooms();
            }
        }

        self.add_corridors();
        self.add_up_and_down_stairs(prev_down_stairs_pos);
    }

    fn add_up_and_down_stairs(&mut self, prev_down_stairs_pos: Option<Point>) {
        // TODO add result with errors
        let random_room = rng::range(0, self.rooms.len() as i32 - 1) as usize;
        let center = self.rooms[random_room].center();
        let index = self.map.xy_to_index(center.0, center.1);
        self.map.tiles[index] = TileType::StairsDown;

        if let Some(prev_stairs) = prev_down_stairs_pos {
            let index = self
                .map
                .xy_to_index(prev_stairs.x as usize, prev_stairs.y as usize);

            if !self.map.tiles[index].blocks_movement() {
                self.map.tiles[index] = TileType::StairsUp;
            }
        }
    }

    fn add_rooms(&mut self) {
        let mut rooms = vec![];
        let rooms_num = rng::range(self.config.rooms_min as i32, self.config.rooms_max as i32);
        while rooms.len() != rooms_num as usize {
            let w = rng::range(
                self.config.room_size_min as i32,
                self.config.room_size_max as i32,
            );
            let h = rng::range(
                self.config.room_size_min as i32,
                self.config.room_size_max as i32,
            );
            let x = rng::range(1, self.width as i32 - 1 - w);
            let y = rng::range(1, self.height as i32 - 1 - h);
            let new_room = Rect::new(x as usize, y as usize, w as usize, h as usize);
            if rooms.iter().all(|room| !new_room.intersect(room)) {
                apply_room_to_map(&new_room, &mut self.map);
                rooms.push(new_room);

                #[cfg(feature = "map_gen_testing")]
                self.history
                    .push((self.map.clone(), "Adding rooms".to_string()));
            }
        }
        self.rooms = rooms;
    }

    fn add_corridors(&mut self) {
        let rooms = self.rooms.clone();
        for (room1, room2) in rooms.iter().tuple_windows() {
            let (new_x, new_y) = room1.center();
            let (prev_x, prev_y) = room2.center();

            if rng::rand_bool() {
                apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
            } else {
                apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
            }

            #[cfg(feature = "map_gen_testing")]
            self.history
                .push((self.map.clone(), "Adding corridors".to_string()));
        }
    }
}

impl MapGenerator for BasicDungeonMap {
    fn generate(&mut self, prev_down_stairs_pos: Option<Point>) -> Result<()> {
        self.create_basic_dungeon_map(prev_down_stairs_pos);
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
    fn history(&self) -> Vec<(Map, String)> {
        self.history.clone()
    }
}
