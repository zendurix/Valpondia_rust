use serde::{Deserialize, Serialize};

use super::{Map, TileType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x1: usize,
    pub x2: usize,
    pub y1: usize,
    pub y2: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        }
    }

    // check if  overlaps with other rect
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn width(&self) -> usize {
        self.x2 - self.x1
    }

    pub fn height(&self) -> usize {
        self.y2 - self.y1
    }

    pub fn area_within(&self) -> Vec<(usize, usize)> {
        let mut area = vec![];
        for y in self.y1 + 1..self.y2 {
            for x in self.x1 + 1..self.x2 {
                area.push((x, y));
            }
        }
        area
    }
}

pub fn apply_room_to_map(room: &Rect, map: &mut Map) {
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            let tile_index = map.xy_to_index(x, y);
            map.tiles[tile_index] = TileType::Floor;
        }
    }
}
