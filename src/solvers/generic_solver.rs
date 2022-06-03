use std::rc::Rc;

use crate::path_node::Node;

use super::path_types::PathResult;

pub trait PathSolver<H> where H: Fn(Node, Node) -> f32 {
    fn calculate(
        &mut self, 
    ) -> PathResult;

    fn reconstruct_path(&self, node: Rc<Node>) -> Vec<Rc<Node>> {
        let mut path = Vec::new();
        let mut tmp = node;
        while let Some(parent) = &tmp.parent {
            let parent = parent.clone();
            path.push(parent.clone());
            tmp = parent;
        }

        path.reverse();
        path
    }
}


