/*
g(p) < h(p)
     ? g(p) + h(p)
     : (g(p) + (2*w - 1) * h(p)) / w */

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{fmt::Debug, hash::Hash};

use super::{Goal, Movements, Node, Pathfinder};

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
        println!("got called");
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

pub struct AStar<F, Pos> {
    closed: HashSet<Pos>,
    heap: BinaryHeap<Node<F, Pos, AStarT<F, Pos>>>,
    open: HashMap<Pos, (F, AStarT<F, Pos>)>,
    refpool: refpool::Pool<NodeLeaf<Pos>>,
    weights: fn(F, F) -> F,
}
impl<F, Pos> AStar<F, Pos> {
    pub fn dbg(&self) {
        dbg!((
            self.refpool.get_max_size() - self.refpool.get_pool_size(),
            self.open.len(),
            self.closed.len(),
            self.heap.len(),
        ));
    }
}
impl<F, Pos> AStar<F, Pos> {
    pub fn with_refpool_size(size: usize, w: fn(F, F) -> F) -> Self
    where
        F: Ord,
    {
        Self {
            closed: Default::default(),
            heap: Default::default(),
            open: Default::default(),
            refpool: refpool::Pool::new(size),
            weights: w,
        }
    }
    fn clear(&mut self) {
        self.closed.clear();
        self.heap.clear();
        self.open.clear();
    }
    fn path(&mut self, end_node: Node<F, Pos, AStarT<F, Pos>>) -> Vec<Pos>
    where
        Pos: Clone,
    {
        // println!("{:?}", &end_node);
        let mut v = vec![end_node.pos];
        if let Some(parent) = end_node.t.parent {
            parent.into_vec(&mut v);
        }
        v
    }
}

impl<F, Pos> Pathfinder for AStar<F, Pos>
where
    Pos: Hash + Eq + Clone + Copy,
    F: core::ops::Add<F, Output = F> + core::ops::Sub<F, Output = F> + Ord + Default + Clone,
{
    type F = F;
    type Pos = Pos;

    fn compute(
        &mut self,
        start: Self::Pos,
        goal: impl Goal<F, Pos>,
        movements: impl Movements<F, Pos>,
    ) -> Option<Vec<Self::Pos>> {
        self.clear();

        let h = goal.heuristic(&start);
        let start_node = Node {
            f: h.clone(),
            pos: start,
            t: AStarT {
                g: F::default(),
                parent: None,
            },
        };
        self.heap.push(start_node.clone());

        let max_cost: F = F::default();
        let mut best_node = start_node;

        while let Some(node) = self.heap.pop() {
            // println!("{:?}", node);
            // println!("{}", self.refpool.get_pool_size());
            if goal.is_reached(&node.pos) {
                let res = self.path(node);
                // self.clear();
                return Some(res);
            }

            self.open.remove(&node.pos);
            self.closed.insert(node.pos);
            // self.visited_chunks.insert(node.pos.into());

            let parent = refpool::PoolRef::new(&self.refpool, NodeLeaf(node.pos, node.t.parent));
            let neighbors = movements.get_neighbors(node.pos);
            for (neighbor_pos, cost) in neighbors {
                if self.closed.contains(&neighbor_pos) {
                    continue;
                }

                let this_g = node.t.g.clone() + cost;
                let heuristic = goal.heuristic(&neighbor_pos);
                let this_f = (self.weights)(this_g.clone(), heuristic.clone());
                if max_cost > F::default() && this_f.clone() > max_cost {
                    continue;
                }

                match self.open.entry(neighbor_pos) {
                    Occupied(mut entry) => {
                        if entry.get().1.g < this_g {
                            continue;
                        }
                        let node = entry.get_mut();
                        let (f, t) = node;
                        *f = this_f.clone();
                        t.g = this_g.clone();
                        t.parent = Some(parent.clone());
                        if this_f - this_g < best_node.f.clone() - best_node.t.g.clone() {
                            best_node = (neighbor_pos, (f.clone(), t.clone())).into();
                        }
                    }
                    Vacant(entry) => {
                        let node = entry.insert((
                            this_f.clone(),
                            AStarT {
                                g: this_g.clone(),
                                parent: Some(parent.clone()),
                            },
                        ));
                        let neighbor = (neighbor_pos, node.clone()).into();
                        if this_f - this_g < best_node.f.clone() - best_node.t.g.clone() {
                            best_node = Node::clone(&neighbor);
                        }
                        self.heap.push(neighbor);
                    }
                };
            }
        }
        None
    }
}
