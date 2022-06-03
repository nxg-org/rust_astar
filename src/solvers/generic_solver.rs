use std::rc::Rc;

use crate::path_node::Node;

use super::path_types::PathResult;

pub trait PathSolver<H> where H: Fn(Node, Node) -> f32 {
    fn calculate(
        &mut self, 
    ) -> PathResult;

    fn reconstruct_path(&self, node: Rc<Node>) -> Vec<(i32, i32)> {
        let mut path = Vec::new();
        let mut tmp = node;
        path.push((tmp.get_x(), tmp.get_y()));
        while let Some(parent) = &tmp.parent {
            let parent = parent.clone();
            path.push((parent.get_x(), parent.get_y()));
            tmp = parent;
        }

        path.reverse();
        path
    }

    fn reconstruct_path1(&self, node: Rc<Node>) -> Vec<Rc<Node>> {
        let mut path = Vec::new();
        let mut tmp = node;
        path.push(tmp.clone());
        while let Some(parent) = &tmp.parent {
            let parent = parent.clone();
            path.push(parent.clone());
            tmp = parent;
        }

        path.reverse();
        path
    }
}


