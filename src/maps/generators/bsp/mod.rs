mod tree;

use rltk::Point;

use crate::{maps::{Error, Map, TileType, rect::{Rect, apply_color_to_walls, apply_room_to_map}}, rng};

use self::tree::{BSPNode, BTree, NodeOrientation};

use super::{
    common::{apply_horizontal_tunnel, apply_vertical_tunnel},
    MapGenerator,
};

use crate::maps::errors::Result;

pub struct BSPConfig {
    pub room_size_min: usize,

    pub tree_height: usize,
}

impl Default for BSPConfig {
    fn default() -> BSPConfig {
        BSPConfig {
            room_size_min: 5,
            tree_height: 4,
        }
    }
}

pub struct BSPMapGen {
    width: usize,
    height: usize,
    config: BSPConfig,

    tree: BTree,

    map: Map,
    rooms: Vec<Rect>,

    #[cfg(feature = "map_gen_testing")]
    history: Vec<Map>,
}

impl BSPMapGen {
    pub fn new(width: usize, height: usize, config: BSPConfig) -> BSPMapGen {
        BSPMapGen {
            width,
            height,
            config,
            map: Map::new(width, height).with_all_solid(),
            rooms: vec![],

            #[cfg(feature = "map_gen_testing")]
            history: vec![],
            tree: BTree::default(),
        }
    }

    fn make_tree(&mut self) -> Result<()> {
        self.tree.nodes.push(BSPNode::new(
            0,
            0,
            0,
            0,
            Rect::new(0, 0, self.width - 1, self.height - 1),
        ));

        let mut current_parent = 0;

        for tree_level in 0..self.config.tree_height {
            let nodes_num = (2_u32).pow(tree_level as u32);

            for _ in 0..nodes_num {
                println!(
                    "current level {}: current_parent {}",
                    tree_level, current_parent
                );
                if self.split_node(current_parent, tree_level).is_err() {
                    // TODO temp break if too many retires, but dont return error
                    return Ok(());
                }
                current_parent += 1;
            }
        }

        Ok(())
    }

    fn split_node(&mut self, parent_node: usize, tree_level: usize) -> Result<()> {
        let orientation = NodeOrientation::rand();

        let (rect1, rect2) = self.try_split(parent_node, orientation)?;

        #[cfg(feature = "map_gen_testing")]
        {
            apply_color_to_walls(&rect1, &mut self.map);
            apply_color_to_walls(&rect2, &mut self.map);

            self.history.push(self.map.clone());
        }

        let last_index = self.tree.nodes.len() - 1;

        let index1 = last_index + 1;
        let index2 = last_index + 2;

        let child1 = BSPNode::new(index1, parent_node, index2, tree_level, rect1);
        let child2 = BSPNode::new(index2, parent_node, index1, tree_level, rect2);

        self.tree.nodes.push(child1);
        self.tree.nodes.push(child2);

        self.tree.nodes[parent_node].make_childreen(index1, index2);

        Ok(())
    }

    fn try_split(&self, parent: usize, orientation: NodeOrientation) -> Result<(Rect, Rect)> {
        let min_size = self.config.room_size_min;
        let mut error_count = 0;
        match orientation {
            NodeOrientation::Vertical => {
                loop {
                    let (rect1, rect2) = self
                        .split_horizontal(parent)
                        .or_else(|_| self.split_vertical(parent))?;

                    if rect1.width() > min_size
                        && rect2.width() > min_size
                        && rect1.height() > min_size
                        && rect2.height() > min_size
                    {
                        return Ok((rect1, rect2));
                    }
                    error_count += 1;

                    // TODO handle this, or maybe add to config
                    if error_count > 200 {
                        return Err(Error::TooManyBSPSplitRetries);
                    }
                }
            }
            NodeOrientation::Horizontal => {
                loop {
                    let (rect1, rect2) = self
                        .split_vertical(parent)
                        .or_else(|_| self.split_horizontal(parent))?;

                    if rect1.height() > min_size
                        && rect2.height() > min_size
                        && rect1.width() > min_size
                        && rect2.width() > min_size
                    {
                        return Ok((rect1, rect2));
                    }
                    error_count += 1;

                    // TODO handle this, or maybe add to config
                    if error_count > 200 {
                        return Err(Error::TooManyBSPSplitRetries);
                    }
                }
            }
        }
    }

    fn split_horizontal(&self, parent: usize) -> Result<(Rect, Rect)> {
        let parent = &self.tree.nodes[parent];
        if parent.area.height() < self.config.room_size_min * 2 + 2 {
            return Err(Error::TooSmallBSPAreaToSplit {});
        }

        let min_y = parent.area.y1 + (parent.area.height() / 2) - (self.config.room_size_min / 2);
        let max_y = parent.area.y1 + (parent.area.height() / 2) + (self.config.room_size_min / 2);
        let y = rng::range(min_y as i32, max_y as i32) as usize;

        let width = parent.area.width();
        let height1 = y - parent.area.y1;
        let height2 = parent.area.height() - height1;

        // TODO maybe add 1 to y in rects
        Ok((
            Rect::new(parent.area.x1, parent.area.y1, width, height1),
            Rect::new(parent.area.x1, y, width, height2),
        ))
    }

    fn split_vertical(&self, parent: usize) -> Result<(Rect, Rect)> {
        let parent = &self.tree.nodes[parent];
        if parent.area.width() < self.config.room_size_min * 2 + 2 {
            return Err(Error::TooSmallBSPAreaToSplit {});
        }

        let min_x = parent.area.x1 + (parent.area.width() / 2) - (self.config.room_size_min / 2);
        let max_x = parent.area.x1 + (parent.area.width() / 2) + (self.config.room_size_min / 2);
        let x = rng::range(min_x as i32, max_x as i32) as usize;

        let height = parent.area.height();
        let width1 = x - parent.area.x1;
        let width2 = parent.area.width() - width1;

        // TODO maybe add 1 to x in rects
        Ok((
            Rect::new(parent.area.x1, parent.area.y1, width1, height),
            Rect::new(x, parent.area.y1, width2, height),
        ))
    }

    fn fill_tree_leaves_with_rooms(&mut self) {
        let mut rooms = vec![];

        println!("tree len: {}", self.tree.nodes.len());

        for node in self.tree.nodes.iter_mut() {
            if node.childreen.is_none() {
                println!("childless node: {}", node.index);
                let room = node.make_random_room(self.config.room_size_min);
                rooms.push(room);
            }
        }
        for room in rooms.iter() {
            apply_room_to_map(room, &mut self.map);

            #[cfg(feature = "map_gen_testing")]
            self.history.push(self.map.clone());
        }
        self.rooms = rooms;
    }

    fn connect_rooms(&mut self) {
        let tree_height = self.config.tree_height;

        let additional_connections = 3;
        let mut additional_connections_counter = 0;

        for tree_level in (1..=tree_height).rev() {
            let start_node = ((2_u32).pow(tree_level as u32) - 1) as usize;
            let max_node = ((2_u32).pow(tree_height as u32 + 1) - 2) as usize;

            let i_adder = |counter: usize| {
                if tree_level != 1 {
                    2
                } else {
                    if counter < additional_connections {
                        0
                    } else {
                        2
                    }
                }
            } as usize;

            let mut i = start_node;
            while i < max_node {
                if tree_level == 1 {
                    additional_connections_counter += 1;
                }

                let mut found_route = false;

                let mut node1 = &self.tree.nodes[i];
                let mut node2 = &self.tree.nodes[i + 1];

                while !found_route {
                    node1 = &self.tree.nodes[i];
                    node2 = &self.tree.nodes[i + 1];

                    while node1.childreen.is_some() {
                        let rand_child = rng::range(0, 1) as usize;
                        let child1 = node1.childreen.unwrap()[rand_child];
                        node1 = &self.tree.nodes[child1];
                    }
                    while node2.childreen.is_some() {
                        let rand_child = rng::range(0, 1) as usize;
                        let child2 = node2.childreen.unwrap()[rand_child];
                        node2 = &self.tree.nodes[child2];
                    }
                }

                i += i_adder(additional_connections_counter);
            }
        }
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
}

impl MapGenerator for BSPMapGen {
    fn generate(&mut self, prev_down_stairs_pos: Option<Point>) -> Result<()> {
        self.map = Map::new(self.width, self.height).with_all_solid();

        let mut errors_count = 0;

        while let Err(e) = self.make_tree() {
            println!("RESET ________________");
            self.reset();
            errors_count += 1;

            if errors_count > 20 {
                return Err(e);
            }
        }
        self.fill_tree_leaves_with_rooms();
        //  self.connect_rooms();
        self.add_up_and_down_stairs(prev_down_stairs_pos);

        // conect
        // add stairs

        Ok(())
    }

    fn reset(&mut self) {
        #[cfg(feature = "map_gen_testing")]
        self.history.clear();

        self.rooms.clear();
        self.tree.nodes.clear();
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
