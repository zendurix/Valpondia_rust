use rltk::Point;

use crate::{maps::rect::Rect, rng};

#[derive(Default)]
pub struct BTree {
    pub nodes: Vec<BSPNode>,
}

impl BTree {
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
        self.room = Some(room.clone());
        room
    }
}
