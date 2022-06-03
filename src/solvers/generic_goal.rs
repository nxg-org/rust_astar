use crate::{world_info::Block, path_node::Node};

pub trait GenericGoal {
    fn goal_reached(&self, other: impl Into<Block>) -> bool;

    fn heuristic(&self, other: impl Into<Block>) -> f32;
}



pub struct GoalNode(pub Node);

impl <'a> GenericGoal for GoalNode {
    fn goal_reached(&self, other: impl Into<Block>) -> bool {
        self.0.data == other.into()
    }

    fn heuristic(&self, other:impl Into<Block>) -> f32 {
        let other: Block = other.into();
        (((other.x - self.0.data.x).pow(2) + (other.x - self.0.data.y).pow(2)) as f32).sqrt()
    }
}