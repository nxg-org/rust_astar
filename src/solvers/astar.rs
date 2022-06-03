use std::{collections::{
    hash_map::Entry::{Occupied, Vacant},
    BinaryHeap, HashMap, HashSet,
}, time::Instant, rc::Rc};

use crate::{path_node::Node, world_info::Block};

use super::{
    generic_goal::{GenericGoal, GoalNode},
    generic_solver::PathSolver,
    path_types::PathResult,
};

pub struct AStarSolver<H, Goal>  {
    pub goal: Goal,
    pub open_heap: BinaryHeap<Node>,
    pub open_block_map: HashMap<Block, Node>,
    pub closed_block_set: HashSet<Block>,
    pub best_node: Node,
    pub heuristic: H,
    pub max_cost: f32,
    pub timeout: u128,
}

impl< H, Goal> AStarSolver< H, Goal> where Goal: GenericGoal {
    fn new(start: Node, goal: Goal, heuristic: H, max_cost: f32, timeout: u128) -> Self {
        let mut open_heap = BinaryHeap::new();
        let mut open_block_map = HashMap::new();
        open_heap.push(start.clone());
        open_block_map.insert(start.data, start.clone());
        Self {
            goal,
            open_heap,
            open_block_map,
            closed_block_set: Default::default(),
            best_node: start,
            heuristic,
            max_cost,
            timeout,
        }
    }
}

impl<H, Goal> PathSolver<H> for AStarSolver<H, Goal>
where
    H: Fn(Node, Node) -> f32,
    Goal: GenericGoal 
{
    fn calculate(&mut self) -> PathResult {

        let start = Instant::now();
        while let Some(node) = self.open_heap.pop() {
            if self.goal.goal_reached(&node) {
                return PathResult::Complete(self.reconstruct_path(Rc::new(node)));
            }

            if (Instant::now() - start).as_millis() > self.timeout {
                return PathResult::Timeout(self.reconstruct_path(Rc::new(self.best_node.clone())));
            }

            // else
            self.open_block_map.remove(&node.data);
            self.closed_block_set.insert(node.data);

            fn get_neighbors(org: Block) -> Vec<(f32, Block)> {
                todo!()
            }

            for (cost, neighbor_block) in get_neighbors(node.data) {
                if self.closed_block_set.contains(&neighbor_block) {
                    continue;
                }
                // else
                let this_g = node.g + cost;
                let heuristic = self.goal.heuristic(neighbor_block);
                if self.max_cost > 0.0 && this_g + heuristic > self.max_cost {
                    continue;
                }

                match self.open_block_map.entry(neighbor_block) {
                    Occupied(mut entry) => {
                        if entry.get().g < this_g {
                            continue;
                        }
                        let node = entry.get_mut();
                        node.f = this_g + heuristic;
                        node.g = this_g;
                        node.h = heuristic;
                        if heuristic < self.best_node.h {
                            self.best_node = node.clone()
                        }
                    }
                    Vacant(entry) => {
                        let neighbor = entry.insert(Node::with_costs_and_data(neighbor_block, this_g, heuristic, Some(Rc::new(node.clone()))));
                        if heuristic < self.best_node.h {
                            self.best_node = neighbor.clone();
                        }
                        self.open_heap.push(neighbor.clone());
                    }
                };
            }
        }

        return PathResult::NoPathResolution(self.reconstruct_path(Rc::new(self.best_node.clone())));

    }
}
