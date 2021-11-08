use rltk::{Algorithm2D, BaseMap, FontCharType, RGB};
use serde::{Deserialize, Serialize};
use specs::Entity;

use super::rect::Rect;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Floor,
    Wall,
    StairsDown,
    StairsUp,
}

impl TileType {
    pub fn blocks_visibility(self) -> bool {
        self == TileType::Wall
    }

    pub fn blocks_movement(self) -> bool {
        self == TileType::Wall
    }

    pub fn draw(self) -> (FontCharType, RGB) {
        match self {
            TileType::Floor => (rltk::to_cp437('.'), RGB::named(rltk::GREEN)),
            TileType::Wall => (rltk::to_cp437('#'), RGB::named(rltk::GREEN)),
            TileType::StairsDown => (rltk::to_cp437('>'), RGB::named(rltk::PINK2)),
            TileType::StairsUp => (rltk::to_cp437('<'), RGB::named(rltk::PINK2)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: usize,
    pub height: usize,
    pub rooms: Vec<Rect>,
    /// by index
    pub blocked: Vec<bool>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let map_len = width * height;
        let tiles = vec![TileType::Floor; map_len];

        Map {
            width,
            height,
            tiles,
            rooms: vec![],
            blocked: vec![false; width * height],
            tile_content: vec![vec![]; width * height],
        }
    }

    pub fn with_all_solid(mut self) -> Map {
        for tile in self.tiles.iter_mut() {
            *tile = TileType::Wall;
        }
        self
    }

    pub fn update_blocked_with_blocking_tiles(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocked[i] = tile.blocks_movement();
        }
    }

    pub fn with_edges_solid(mut self) -> Map {
        for x in 0..self.width {
            // upper
            let map_index = self.xy_to_index(x, 0);
            self.tiles[map_index] = TileType::Wall;

            // lower
            let map_index = self.xy_to_index(x, self.height_max());
            self.tiles[map_index] = TileType::Wall;
        }
        for y in 0..self.height {
            // left
            let map_index = self.xy_to_index(0, y);
            self.tiles[map_index] = TileType::Wall;

            // right
            let map_index = self.xy_to_index(self.width_max(), y);
            self.tiles[map_index] = TileType::Wall;
        }

        self
    }

    pub fn tile_at_xy(&self, x: usize, y: usize) -> TileType {
        let map_index = self.xy_to_index(x, y);
        self.tiles[map_index]
    }

    pub fn try_get_tile_at_xy(&self, x: usize, y: usize) -> Option<TileType> {
        if x > self.width_max() || x > self.height_max() {
            return None;
        }
        let map_index = self.xy_to_index(x, y);
        Some(self.tiles[map_index])
    }

    pub fn set_tile_at_index(&mut self, index: usize, tile_type: TileType) {
        if index < self.index_max() {
            self.tiles[index] = tile_type;
        }
    }

    pub fn width_max(&self) -> usize {
        self.width - 1
    }
    pub fn height_max(&self) -> usize {
        self.height - 1
    }
    pub fn index_max(&self) -> usize {
        self.height * self.width
    }

    pub fn tiles(&self) -> &[TileType] {
        &self.tiles
    }

    pub fn xy_to_index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    pub fn index_to_xy(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    fn is_exit_valid(&self, x: usize, y: usize) -> bool {
        if x < 1 || x > self.width_max() || y < 1 || y > self.height_max() {
            return false;
        }
        !self.blocked[self.xy_to_index(x, y)]
    }

    pub fn clear_tiles_contents(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].blocks_visibility()
    }

    fn get_pathing_distance(&self, index1: usize, index2: usize) -> f32 {
        let w = self.width;
        let p1 = rltk::Point::new(index1 % w, index1 / w);
        let p2 = rltk::Point::new(index2 % w, index2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, index: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = index % self.width;
        let y = index / self.width;
        let w = self.width;

        if self.is_exit_valid(x - 1, y) {
            exits.push((index - 1, 1.0));
        }
        if self.is_exit_valid(x + 1, y) {
            exits.push((index + 1, 1.0));
        }
        if self.is_exit_valid(x, y - 1) {
            exits.push((index - w, 1.0));
        }
        if self.is_exit_valid(x, y + 1) {
            exits.push((index + w, 1.0));
        }

        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((index - 1) - w, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((index + 1) - w, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((index + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((index + w) + 1, 1.45));
        }

        exits
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> rltk::Point {
        rltk::Point::new(self.width, self.height)
    }
}
