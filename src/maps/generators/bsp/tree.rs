use crate::maps::errors::Result;
use crate::{
    maps::{rect::Rect, Error, Map},
    rng,
};

#[derive(Default)]
pub struct BTree {
    pub nodes: Vec<BSPNode>,

    #[cfg(feature = "map_gen_testing")]
    pub split_history: Vec<(Map, String)>,
}

impl BTree {
    #[cfg(feature = "map_gen_testing")]
    fn save_split_to_history(
        &mut self,
        rect1: &Rect,
        rect2: &Rect,
        splittend_node_index: usize,
        tree_level: usize,
    ) {
        use crate::maps::rect::apply_color_to_walls;

        let mut last_map_state = self.split_history.last().unwrap().clone();

        apply_color_to_walls(rect1, &mut last_map_state.0);
        apply_color_to_walls(rect2, &mut last_map_state.0);

        self.split_history.push((
            last_map_state.0,
            format!(
                "Splitting node: {} at tree level: {}",
                splittend_node_index, tree_level
            ),
        ));
    }

    pub fn make_tree(
        &mut self,
        map_width: usize,
        map_height: usize,
        tree_height: usize,
        min_area_size: usize,
    ) -> Result<()> {
        let map_rect = Rect::new(0, 0, map_width - 1, map_height - 1);

        #[cfg(feature = "map_gen_testing")]
        {
            let map = Map::new(map_width, map_height).with_all_solid();
            self.split_history = vec![(map, "Start".to_string())];
            self.save_split_to_history(&map_rect, &map_rect, 0, 0)
        }

        self.nodes.push(BSPNode::new(0, 0, 0, 0, map_rect));

        let mut current_parent = 0;

        for tree_level in 0..tree_height {
            let nodes_num = (2_u32).pow(tree_level as u32);

            for _ in 0..nodes_num {
                if self
                    .split_node(current_parent, tree_level, min_area_size)
                    .is_err()
                {
                    // TODO temp break if too many retires, but dont return error
                    return Ok(());
                }
                current_parent += 1;
            }
        }

        Ok(())
    }

    fn split_node(
        &mut self,
        parent_node: usize,
        tree_level: usize,
        min_area_size: usize,
    ) -> Result<()> {
        let orientation = NodeOrientation::rand();

        let (rect1, rect2) = self.try_split(parent_node, orientation, min_area_size)?;

        #[cfg(feature = "map_gen_testing")]
        self.save_split_to_history(&rect1, &rect2, parent_node, tree_level);

        let last_index = self.nodes.len() - 1;

        let index1 = last_index + 1;
        let index2 = last_index + 2;

        let child1 = BSPNode::new(index1, parent_node, index2, tree_level, rect1);
        let child2 = BSPNode::new(index2, parent_node, index1, tree_level, rect2);

        self.nodes.push(child1);
        self.nodes.push(child2);

        self.nodes[parent_node].make_childreen(index1, index2);

        Ok(())
    }

    fn try_split(
        &self,
        parent: usize,
        orientation: NodeOrientation,
        min_area_size: usize,
    ) -> Result<(Rect, Rect)> {
        let mut error_count = 0;
        match orientation {
            NodeOrientation::Vertical => {
                loop {
                    let (rect1, rect2) = self
                        .split_horizontal(parent, min_area_size)
                        .or_else(|_| self.split_vertical(parent, min_area_size))?;

                    if rect1.width() > min_area_size
                        && rect2.width() > min_area_size
                        && rect1.height() > min_area_size
                        && rect2.height() > min_area_size
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
                        .split_vertical(parent, min_area_size)
                        .or_else(|_| self.split_horizontal(parent, min_area_size))?;

                    if rect1.height() > min_area_size
                        && rect2.height() > min_area_size
                        && rect1.width() > min_area_size
                        && rect2.width() > min_area_size
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

    fn split_horizontal(&self, parent: usize, min_area_size: usize) -> Result<(Rect, Rect)> {
        let parent = &self.nodes[parent];
        if parent.area.height() < min_area_size * 2 + 2 {
            return Err(Error::TooSmallBSPAreaToSplit {});
        }

        let min_y = parent.area.y1 + (parent.area.height() / 2) - (min_area_size / 2);
        let max_y = parent.area.y1 + (parent.area.height() / 2) + (min_area_size / 2);
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

    fn split_vertical(&self, parent: usize, min_area_size: usize) -> Result<(Rect, Rect)> {
        let parent = &self.nodes[parent];
        if parent.area.width() < min_area_size * 2 + 2 {
            return Err(Error::TooSmallBSPAreaToSplit {});
        }

        let min_x = parent.area.x1 + (parent.area.width() / 2) - (min_area_size / 2);
        let max_x = parent.area.x1 + (parent.area.width() / 2) + (min_area_size / 2);
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

    pub fn node_family(&self, node_index: usize) -> Vec<usize> {
        let mut family = vec![];
        let mut act_node_index = node_index;

        while act_node_index != 0 {
            family.push(act_node_index);

            act_node_index = self.nodes[act_node_index].parent;
        }
        family.push(act_node_index);

        family
    }

    pub fn node_children(&self, node_index: usize) -> Vec<usize> {
        let mut childreen = vec![];
        let act_node = &self.nodes[node_index];

        if let Some(node_childreen) = act_node.childreen {
            childreen.push(node_childreen[0]);
            childreen.push(node_childreen[1]);
            childreen.extend(self.node_children(node_childreen[0]));
            childreen.extend(self.node_children(node_childreen[1]));
        }
        childreen
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NodeOrientation {
    Horizontal,
    Vertical,
}

impl NodeOrientation {
    pub fn rand() -> NodeOrientation {
        if rng::rand_bool() {
            NodeOrientation::Horizontal
        } else {
            NodeOrientation::Vertical
        }
    }
}

#[derive(Clone)]
pub struct BSPNode {
    pub index: usize,
    /// 0 is root of tree
    pub tree_level: usize,

    /// index
    pub parent: usize,
    /// index
    pub sister: usize,
    /// indexes (Left, Right) / (Up, Down)
    pub childreen: Option<[usize; 2]>,

    /// indexes
    pub family: Vec<usize>,

    pub area: Rect,
    pub room: Option<Rect>,
}

impl BSPNode {
    pub fn new(
        index: usize,
        parent: usize,
        sister: usize,
        tree_level: usize,
        area: Rect,
    ) -> BSPNode {
        BSPNode {
            index,
            tree_level,
            parent,
            sister,
            childreen: None,
            family: vec![],
            area,
            room: None,
        }
    }

    pub fn make_childreen(&mut self, child1: usize, child2: usize) {
        self.childreen = Some([child1, child2]);
    }

    pub fn make_random_room(&mut self, min_size: usize) -> Rect {
        let min_size = if self.area.width() > (min_size * 3) + 3
            && self.area.height() > (min_size * 3) + 3
        {
            min_size * 3
        } else if self.area.width() > (min_size * 2) + 3 && self.area.height() > (min_size * 2) + 3
        {
            min_size * 2
        } else {
            min_size
        };

        let width = rng::range(min_size as i32, self.area.width() as i32) as usize;
        let height = rng::range(min_size as i32, self.area.height() as i32) as usize;

        let max_x = self.area.x2 - width;
        let max_y = self.area.y2 - height;

        let x = rng::range(self.area.x1 as i32, max_x as i32) as usize;
        let y = rng::range(self.area.y1 as i32, max_y as i32) as usize;

        let room = Rect::new(x, y, width, height);
        self.room = Some(room);
        room
    }
}
