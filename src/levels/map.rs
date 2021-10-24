use rltk::Tile;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let map_len = width * height;
        let tiles = vec![TileType::Floor; map_len];

        Map {
            width,
            height,
            tiles,
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

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }
}
