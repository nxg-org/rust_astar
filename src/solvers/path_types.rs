use std::rc::Rc;

use crate::path_node::Node;


type Test = Vec<(i32, i32)>;

//Vec<Rc<Node>>;// 

#[derive(Debug)]
pub enum PathResult {
    Partial(Test),
    Complete(Test),
    Timeout(Test),
    NoPathResolution(Test)
}