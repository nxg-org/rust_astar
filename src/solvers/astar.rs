use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BinaryHeap, HashMap, HashSet,
    },
    rc::Rc,
    time::Instant,
};

use crate::{
    constants::{CARDINAL_POSITIONS, DIAGONAL_POSITIONS},
    path_node::Node,
    world_info::{Block, Grid},
};

use super::{
    generic_goal::{GenericGoal, GoalNode},
    generic_solver::PathSolver,
    path_types::PathResult,
};

pub struct AStarSolver<H, Goal> {
    pub goal: Goal,
    pub open_heap: BinaryHeap<Node>,
    pub open_block_map: HashMap<Block, Node>,
    pub closed_block_set: HashSet<Block>,
    pub best_node: Node,
    pub heuristic: H,
    pub max_cost: f32,
    pub timeout: u128,

    // this is bad.
    pub grid: Grid,
}

/// Laziness.
/// We're going to implement getting neighbors lazily here in favor of testing.
/// In reality, this will be **much** more complicated.
/// Here goes.
impl<H, Goal> AStarSolver<H, Goal>
where
    H: Fn(Node, Node) -> f32,
    Goal: GenericGoal,
{
    pub fn new(
        start: Node,
        goal: Goal,
        heuristic: H,
        max_cost: f32,
        timeout: u128,
        /* this is bad. */ grid: Grid,
    ) -> Self {
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
            grid,
        }
    }

    fn get_neighbors(&self, org: Block) -> Vec<(f32, Block)> {
        const SQRT_2: f32 = 1.41414;

        let mut moves = Vec::with_capacity(CARDINAL_POSITIONS.len() + DIAGONAL_POSITIONS.len());
        for dir in CARDINAL_POSITIONS.iter() {
            if let Some(block) = self.grid.get_pos_info_ref(org.x + dir.0, org.y + dir.1) {
                moves.push((if block.passable { 1f32 } else { f32::MAX }, *block))
            }
        }

        for dir in DIAGONAL_POSITIONS.iter() {
            if let Some(block) = self.grid.get_pos_info_ref(org.x + dir.0, org.y + dir.1) {
                moves.push((if block.passable { SQRT_2 } else { f32::MAX }, *block))
            }
        }
        moves
    }
}

impl<H, Goal> PathSolver<H> for AStarSolver<H, Goal>
where
    H: Fn(Node, Node) -> f32,
    Goal: GenericGoal,
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

            // TODO: update this. Move this ASAP. Hate this.
            for (cost, neighbor_block) in self.get_neighbors(node.data) {
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
                        node.set_costs(this_g, heuristic);
                        if heuristic < self.best_node.h {
                            self.best_node = node.clone()
                        }
                    }
                    Vacant(entry) => {
                        let neighbor = entry.insert(Node::with_costs_and_data(
                            neighbor_block,
                            this_g,
                            heuristic,
                            Some(Rc::new(node.clone())),
                        ));
                        if heuristic < self.best_node.h {
                            self.best_node = neighbor.clone();
                        }
                        self.open_heap.push(neighbor.clone());
                    }
                };
            }
        }

        return PathResult::NoPathResolution(
            self.reconstruct_path(Rc::new(self.best_node.clone())),
        );
    }
}
