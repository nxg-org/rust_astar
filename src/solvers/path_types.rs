use std::rc::Rc;

use crate::path_node::Node;

pub enum PathResult {
    Partial(Vec<Rc<Node>>),
    Complete(Vec<Rc<Node>>),
    Timeout(Vec<Rc<Node>>),
    NoPathResolution(Vec<Rc<Node>>)
}