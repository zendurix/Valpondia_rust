use rltk::Point;

use crate::maps::rect::Rect;


#[derive(Default)]
pub struct BTree {
    pub nodes: Vec<BSPNode>
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
}



#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NodeOrientation {
    Horizontal,
    Vertical
}





#[derive(Clone)]
struct BSPNode {
    pub index: usize,
    /// 0 is root of tree
    pub tree_depth: usize,

    /// index
    pub parent: usize,
    /// index
    pub sister: usize,
    /// indexes (Left, Right) TODO for sure?
    pub childreen: Option<(usize, usize)>,

    /// indexes 
    pub family: Vec<usize>,


    pub area: Rect,

    pub orientation: NodeOrientation,

}


impl BSPNode {
    pub fn new(index: usize, parent: usize, sister: usize, origin: Point, width: usize, height: usize, orientation: NodeOrientation) -> BSPNode {

        let tree_depth = ((index +1) as f32).sqrt() as usize;



        BSPNode {
            index,
            tree_depth,
            parent,
            sister,
            childreen: None,
            family: vec![],
            area: Rect::new(origin.x as usize,origin.y as usize,width,height),
            orientation,
        }
    }


}

