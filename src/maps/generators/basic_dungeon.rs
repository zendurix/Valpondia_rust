use itertools::Itertools;

use crate::{
    maps::{
        corridor::{apply_horizontal_tunnel, apply_vertical_tunnel},
        errors::Result,
        rect::{apply_room_to_map, Rect},
        Map, MapGenerator,
    },
    rng,
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
            rooms_min: 20,
            rooms_max: 40,
            room_size_min: 3,
            room_size_max: 20,
        }
    }
}

pub struct BasicDungeonMap {
    width: usize,
    height: usize,
    config: BasicDungeonMapConfig,
}

impl BasicDungeonMap {
    pub fn new(width: usize, height: usize, config: BasicDungeonMapConfig) -> BasicDungeonMap {
        BasicDungeonMap {
            width,
            height,
            config,
        }
    }

    pub fn create_basic_dungeon_map(&mut self) -> Map {
        let mut map = Map::new(self.width, self.height).with_all_solid();
        self.add_rooms(&mut map);
        self.add_corridors(&mut map);
        map
    }

    fn add_rooms(&mut self, map: &mut Map) {
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
                apply_room_to_map(&new_room, map);
                rooms.push(new_room);
            }
        }
        map.rooms = rooms;
    }

    fn add_corridors(&mut self, map: &mut Map) {
        let rooms = map.rooms.clone();
        for (room1, room2) in rooms.iter().tuple_windows() {
            let (new_x, new_y) = room1.center();
            let (prev_x, prev_y) = room2.center();

            if rng::range(0, 2) == 1 {
                apply_horizontal_tunnel(map, prev_x, new_x, prev_y);
                apply_vertical_tunnel(map, prev_y, new_y, new_x);
            } else {
                apply_vertical_tunnel(map, prev_y, new_y, prev_x);
                apply_horizontal_tunnel(map, prev_x, new_x, new_y);
            }
        }
    }
}

impl MapGenerator for BasicDungeonMap {
    fn generate(mut self) -> Result<Map> {
        Ok(self.create_basic_dungeon_map())
    }
}
