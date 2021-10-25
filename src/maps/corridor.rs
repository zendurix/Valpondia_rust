use super::{Map, TileType};

pub fn apply_horizontal_tunnel(map: &mut Map, x1: usize, x2: usize, y: usize) {
    for x in x1.min(x2)..=x1.max(x2) {
        let tile_index = map.xy_to_index(x, y);
        map.tiles[tile_index] = TileType::Floor;
    }
}

pub fn apply_vertical_tunnel(map: &mut Map, y1: usize, y2: usize, x: usize) {
    for y in y1.min(y2)..=y1.max(y2) {
        let tile_index = map.xy_to_index(x, y);
        map.tiles[tile_index] = TileType::Floor;
    }
}
