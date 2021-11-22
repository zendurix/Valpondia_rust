mod tree;

use rltk::Point;

use crate::{
    maps::{
        rect::{apply_room_to_map, Rect},
        Map, TileType,
    },
    rng,
};

use self::tree::{BTree, NodeOrientation};

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

    fn fill_tree_leaves_with_rooms(&mut self) {
        let mut rooms = vec![];

        for node in self.tree.nodes.iter_mut() {
            if node.childreen.is_none() {
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

                let parent1 = &self.tree.nodes[i];
                let parent2 = &self.tree.nodes[i + 1];

                let split_orientation = if parent1.area.x1 == parent2.area.x1 {
                    NodeOrientation::Horizontal
                } else {
                    NodeOrientation::Vertical
                };

                // RANDOM CHILDREEN
                // while node1.childreen.is_some() {
                //     let rand_child = rng::range(0, 1) as usize;
                //     let child1 = node1.childreen.unwrap()[rand_child];
                //     node1 = &self.tree.nodes[child1];
                // }
                // while node2.childreen.is_some() {
                //     let rand_child = rng::range(0, 1) as usize;
                //     let child2 = node2.childreen.unwrap()[rand_child];
                //     node2 = &self.tree.nodes[child2];
                // }

                let childreen1 = self.tree.node_children(parent1.index);
                let childreen2 = self.tree.node_children(parent2.index);

                let (index1, index2) = match split_orientation {
                    NodeOrientation::Horizontal => (
                        *childreen1
                            .iter()
                            .filter(|ind| self.tree.nodes[**ind].room.is_some())
                            .max_by(|ind1, ind2| {
                                self.tree.nodes[**ind1]
                                    .room
                                    .unwrap()
                                    .y2
                                    .cmp(&self.tree.nodes[**ind2].room.unwrap().y2)
                            })
                            .unwrap_or(&parent1.index),
                        *childreen2
                            .iter()
                            .filter(|ind| self.tree.nodes[**ind].room.is_some())
                            .min_by(|ind1, ind2| {
                                self.tree.nodes[**ind1]
                                    .room
                                    .unwrap()
                                    .y1
                                    .cmp(&self.tree.nodes[**ind2].room.unwrap().y1)
                            })
                            .unwrap_or(&parent2.index),
                    ),
                    NodeOrientation::Vertical => (
                        *childreen1
                            .iter()
                            .filter(|ind| self.tree.nodes[**ind].room.is_some())
                            .max_by(|ind1, ind2| {
                                self.tree.nodes[**ind1]
                                    .room
                                    .unwrap()
                                    .x2
                                    .cmp(&self.tree.nodes[**ind2].room.unwrap().x2)
                            })
                            .unwrap_or(&parent1.index),
                        *childreen2
                            .iter()
                            .filter(|ind| self.tree.nodes[**ind].room.is_some())
                            .min_by(|ind1, ind2| {
                                self.tree.nodes[**ind1]
                                    .room
                                    .unwrap()
                                    .x1
                                    .cmp(&self.tree.nodes[**ind2].room.unwrap().x1)
                            })
                            .unwrap_or(&parent2.index),
                    ),
                };

                let room1 = &self.tree.nodes[index1].room.unwrap();
                let room2 = &self.tree.nodes[index2].room.unwrap();

                let (new_x, new_y) = room1.center();
                let (prev_x, prev_y) = room2.center();

                match split_orientation {
                    NodeOrientation::Horizontal => {
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
                    }
                    NodeOrientation::Vertical => {
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                    }
                }

                let family1 = self.tree.node_family(index1);
                let family2 = self.tree.node_family(index2);

                println!("\nConnecting: ");
                print!("\t1: ");
                for f in family1 {
                    print!("{} ", f);
                }
                println!();
                print!("\t2: ");
                for f in family2 {
                    print!("{} ", f);
                }

                #[cfg(feature = "map_gen_testing")]
                self.history.push(self.map.clone());

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

        while let Err(e) = self.tree.make_tree(
            self.width,
            self.height,
            self.config.tree_height,
            self.config.room_size_min,
        ) {
            println!("RESET ________________");
            self.reset();
            errors_count += 1;

            if errors_count > 20 {
                return Err(e);
            }
        }

        #[cfg(feature = "map_gen_testing")]
        self.history = self.tree.split_history.clone();

        self.fill_tree_leaves_with_rooms();
        self.connect_rooms();
        self.add_up_and_down_stairs(prev_down_stairs_pos);

        // conect
        // add stairs

        Ok(())
    }

    fn reset(&mut self) {
        #[cfg(feature = "map_gen_testing")]
        {
            self.history.clear();
            self.tree.split_history.clear();
        }

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
