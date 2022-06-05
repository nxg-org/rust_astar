use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{fmt::Debug, hash::Hash};

use super::{Goal, Movements, Node, PathResult, PathfinderGen};

#[derive(Clone, Debug)]
pub struct AStarT<F, Pos> {
    g: F,
    parent: Option<refpool::PoolRef<NodeLeaf<Pos>>>,
}

impl<F, Pos> PartialEq for AStarT<F, Pos>
where
    F: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.g == other.g
    }
}
impl<F, Pos> Eq for AStarT<F, Pos> where F: PartialEq {}
impl<F, Pos> PartialOrd for AStarT<F, Pos>
where
    F: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.g.partial_cmp(&other.g)
    }
}
impl<F, Pos> Ord for AStarT<F, Pos>
where
    F: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.g.cmp(&other.g)
    }
}

// impl<Pos> Debug for AStarT<Pos> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("AStarT").field("g", &self.g).field("h", &self.h)
//         // .field("parent", &self.parent)
//         .finish()
//     }
// }

#[derive(Clone, Debug)]
struct NodeLeaf<Pos>(Pos, Option<NodeRef<Pos>>);
type NodeRef<Pos> = refpool::PoolRef<NodeLeaf<Pos>>;
impl<Pos> NodeLeaf<Pos>
where
    Pos: Clone,
{
    fn into_vec(&self, v: &mut Vec<Pos>) {
        let mut r = self.1.as_ref();
        v.push(self.0.clone());
        while let Some(re) = r {
            v.push(re.0.clone());
            r = re.1.as_ref();
        }
    }
}

pub struct AStar<F, Pos, G, M> {
    start: Pos,
    goal: G,
    movements: M,
    max_cost: F,
    best_node: Node<F, Pos, AStarT<F, Pos>>,
    heap: BinaryHeap<Node<F, Pos, AStarT<F, Pos>>>,
    open: HashMap<Pos, (F, AStarT<F, Pos>)>,
    closed: HashSet<Pos>,
    refpool: refpool::Pool<NodeLeaf<Pos>>,
}

impl<F, Pos, G, M> AStar<F, Pos, G, M> {
    fn path(&mut self, end_node: Node<F, Pos, AStarT<F, Pos>>) -> Vec<Pos>
    where
        Pos: Clone,
    {
        // println!("{:?}", &end_node);
        let mut v = vec![end_node.pos];
        if let Some(parent) = end_node.t.parent {
            parent.into_vec(&mut v);
        }

        v.reverse();
        v
    }
}

impl<F, Pos, G, M> PathfinderGen for AStar<F, Pos, G, M>
where
    Pos: Hash + Eq + Clone + Copy,
    F: core::ops::Add<F, Output = F> + core::ops::Sub<F, Output = F> + Ord + Default + Clone,
    G: Goal<F, Pos>,
    M: Movements<F, Pos>,
{
    type F = F;
    type Pos = Pos;
    type Movements = M;
    type Goal = G;

    fn new(
        size: usize,
        max_cost: Self::F,
        start: Self::Pos,
        goal: Self::Goal,
        movements: Self::Movements,
    ) -> Self {
        let mut heap = BinaryHeap::new();
        let h = goal.heuristic(&start);
        let start_node = Node {
            f: h.clone(),
            pos: start,
            t: AStarT {
                g: F::default(),
                parent: None,
            },
        };
        heap.push(start_node.clone());
        Self {
            max_cost,
            movements,
            start,
            goal,
            heap,
            best_node: start_node,
            closed: Default::default(),
            open: Default::default(),
            refpool: refpool::Pool::new(size),
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.closed.clear();
        self.heap.clear();
        self.open.clear();
        let h = self.goal.heuristic(&self.start);
        let start_node = Node {
            f: h,
            pos: self.start,
            t: AStarT {
                g: F::default(),
                parent: None,
            },
        };
        self.heap.push(start_node.clone());
        self.best_node = start_node;
    }

    fn compute(&mut self) -> PathResult<Vec<Self::Pos>> {
        while let Some(node) = self.heap.pop() {
            if self.goal.is_reached(&node.pos) {
                self.reset();
                return PathResult::Complete(self.path(node));
            }

            self.open.remove(&node.pos);
            self.closed.insert(node.pos);
            // self.visited_chunks.insert(node.pos.into());

            let parent = refpool::PoolRef::new(&self.refpool, NodeLeaf(node.pos, node.t.parent));
            let neighbors = self.movements.get_neighbors(node.pos);
            for (neighbor_pos, cost) in neighbors {
                if self.closed.contains(&neighbor_pos) {
                    continue;
                }

                let this_g = node.t.g.clone() + cost;

                let heuristic = self.goal.heuristic(&neighbor_pos);
                if self.max_cost > F::default()
                    && this_g.clone() + heuristic.clone() > self.max_cost
                {
                    continue;
                }

                match self.open.entry(neighbor_pos) {
                    Occupied(mut entry) => {
                        if entry.get().1.g < this_g {
                            continue;
                        }
                        let node = entry.get_mut();
                        let (f, t) = node;
                        *f = this_g.clone() + heuristic.clone();
                        t.g = this_g;
                        t.parent = Some(parent.clone());
                        if heuristic < self.best_node.f.clone() - self.best_node.t.g.clone() {
                            self.best_node = (neighbor_pos, (f.clone(), t.clone())).into();
                        }
                    }
                    Vacant(entry) => {
                        let node = entry.insert((
                            this_g.clone() + heuristic.clone(),
                            AStarT {
                                g: this_g,
                                parent: Some(parent.clone()),
                            },
                        ));
                        let neighbor = (neighbor_pos, node.clone()).into();
                        if heuristic < self.best_node.f.clone() - self.best_node.t.g.clone() {
                            self.best_node = Node::clone(&neighbor);
                        }
                        self.heap.push(neighbor);
                    }
                };
            }
        }
        return PathResult::NoPath(self.path(self.best_node.clone()));
    }
}
