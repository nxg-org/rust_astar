use std::{
    cmp::Ordering,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BinaryHeap, HashMap, HashSet,
    },
    hash::Hash,
};

use super::{node::{Node, ChunkPos}, chunk_data::ChunkDataProvider};



pub trait PathSolver<T, Pos: Hash> {
    fn compute(
        &mut self,
        data: impl ChunkDataProvider,
        start: Pos,
        end: Pos,
    ) -> Option<Vec<Node<T, Pos>>>;
}


pub trait Heuristic<Pos> {
    fn is_goal(&self, p: &Pos) -> bool;
    fn heuristic(&self, p: &Pos) -> f32;
}

#[derive(Clone)]
pub struct AStarT {
    // cost
    g: f32,
    // heuristic
    h: f32,
}

pub struct AStar<Pos, H> {
    closed: HashSet<Pos>,
    heap: BinaryHeap<Node<AStarT, Pos>>,
    openmap: HashMap<Pos, (f32, AStarT)>,
    heuristic: H,
    visited_chunks: HashSet<ChunkPos>,
    best_node: Node<AStarT, Pos>,
    max_cost: f32,
}

impl<Pos, H> AStar<Pos, H>
where
    H: Heuristic<Pos>,
{
    pub fn new(start: Pos, heuristic: H, max_cost: f32) -> Self {
        let start_heuristic = heuristic.heuristic(&start);
        Self {
            closed: Default::default(),
            heap: Default::default(),
            openmap: Default::default(),
            heuristic,
            visited_chunks: Default::default(),
            best_node: Node {
                pos: start,
                f: start_heuristic,
                t: AStarT {
                    g: 0.0,
                    h: start_heuristic,
                },
            },
            max_cost,
        }
    }
}

impl<Pos, H> PathSolver<AStarT, Pos> for AStar<Pos, H>
where
    Pos: Hash + Eq + Copy + Into<ChunkPos>,
    H: Heuristic<Pos>,
{
    fn compute(
        &mut self,
        data: impl ChunkDataProvider,
        start: Pos,
        end: Pos,
    ) -> Option<Vec<Node<AStarT, Pos>>> {
        // todo checks for timeout maybe
        // or async cancellation

        while let Some(node) = self.heap.pop() {
            if self.heuristic.is_goal(&node.pos) {
                return todo!("goal result");
            }

            self.openmap.remove(&node.pos);
            self.closed.insert(node.pos);
            self.visited_chunks.insert(node.pos.into());

            fn get_neighbors<Pos>(pos: Pos) -> Vec<(Pos, f32)> {
                todo!()
            }

            let neighbors = get_neighbors(node.pos);
            for (neighbor_pos, cost) in neighbors {
                if self.closed.contains(&neighbor_pos) {
                    continue;
                }

                let this_g = node.t.g + cost;

                let heuristic = self.heuristic.heuristic(&neighbor_pos);
                if self.max_cost > 0.0 && this_g + heuristic > self.max_cost {
                    continue;
                }

                match self.openmap.entry(neighbor_pos) {
                    Occupied(mut entry) => {
                        if entry.get().1.g < this_g {
                            continue;
                        }
                        let node = entry.get_mut();
                        let (f, t) = node;
                        *f = this_g + heuristic;
                        t.g = this_g;
                        t.h = heuristic;
                        if heuristic < self.best_node.t.h {
                            self.best_node = (neighbor_pos, (*f, t.clone())).into();
                        }
                    }
                    Vacant(entry) => {
                        let node = entry.insert((
                            this_g + heuristic,
                            AStarT {
                                g: this_g,
                                h: heuristic,
                            },
                        ));
                        let neighbor: Node<_, _> = (neighbor_pos, node.clone()).into();
                        if heuristic < self.best_node.t.h {
                            self.best_node = neighbor.clone();
                        }
                        self.heap.push(neighbor);
                    }
                };
            }

            // todo checks for timeout maybe
            // or async cancellation
        }

        None
    }
}
