use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{fmt::Debug, hash::Hash};

// extern crate rustc_arena;

// use refpool::{Pool, PoolBox, PoolRef};

use super::{Goal, Movements, Node, Pathfinder};

#[derive(Clone, Debug)]
pub struct AStarT<F, Pos> {
    g: F,
    parent: Option<NodeRef<F, Pos>>,
}

#[derive(Debug)]
pub struct NodeRef<F, Pos>(*const Node<F, Pos, AStarT<F, Pos>>);
impl<F, Pos> Clone for NodeRef<F, Pos> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
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
struct AStarOrd<F, Pos>(*mut Node<F, Pos, AStarT<F, Pos>>);
impl<F, Pos> PartialEq for AStarOrd<F, Pos> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<F, Pos> Eq for AStarOrd<F, Pos> {}
impl<F, Pos> PartialOrd for AStarOrd<F, Pos>
where
    F: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe { &*self.0 }.partial_cmp(unsafe { &*other.0 })
    }
}
impl<F, Pos> Ord for AStarOrd<F, Pos>
where
    F: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { &*self.0 }.cmp(unsafe { &*other.0 })
    }
}

pub struct AStar<F, Pos> {
    arena: bumpalo::Bump,
    heap: BinaryHeap<AStarOrd<F, Pos>>,
    open: HashMap<Pos, *mut Node<F, Pos, AStarT<F, Pos>>>,
    closed: HashSet<Pos>,
    weights: fn(F, F) -> F,
}

impl<F, Pos> AStar<F, Pos> {
    pub fn dbg(&self) {
        dbg!((
            // self.arena.get_max_size() - self.arena.get_pool_size(),
            self.open.len(),
            self.closed.len(),
            self.heap.len(),
        ));
    }
}

impl<F, Pos> AStar<F, Pos> {
    pub fn with_capacity(size: usize, w: fn(F, F) -> F) -> Self
    where
        F: Ord,
    {
        Self {
            closed: Default::default(),
            heap: Default::default(),
            open: Default::default(),
            arena: bumpalo::Bump::with_capacity(size),
            weights: w,
        }
    }
    fn clear(&mut self) {
        self.arena.reset();
        self.closed.clear();
        self.heap.clear();
        self.open.clear();
    }
}

impl<F, Pos> Node<F, Pos, AStarT<F, Pos>>
where
    Pos: Clone,
{
    fn path(&self) -> Vec<Pos> {
        let mut pp = self.t.parent.clone();
        let mut v = vec![self.pos.clone()];
        while let Some(p) = pp {
            let p = unsafe { &*p.0 };
            v.push(p.pos.clone());
            pp = p.t.parent.clone();
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
        goal: impl Goal<Self::F, Self::Pos>,
        movements: impl Movements<Self::F, Self::Pos>,
    ) -> Option<Vec<Self::Pos>> {
        self.clear();

        let h = goal.heuristic(&start);
        let start_node = self.arena.alloc(Node {
            f: h.clone(),
            pos: start,
            t: AStarT {
                g: F::default(),
                parent: None,
            },
        });
        let max_cost = F::default();
        let mut best_node: *const Node<F, Pos, AStarT<F, Pos>> = start_node;
        self.heap.push(AStarOrd(start_node));

        while let Some(node) = self.heap.pop() {
            let node = unsafe { &mut *node.0 };
            if goal.is_reached(&node.pos) {
                // self.dbg();
                return Some(node.path());
            }
            self.open.remove(&node.pos);
            self.closed.insert(node.pos);

            let neighbors = movements.get_neighbors(node.pos);
            let parent = node;
            for (pos, cost) in neighbors {
                if self.closed.contains(&pos) {
                    continue;
                }

                let this_g = parent.t.g.clone() + cost;
                let heuristic = goal.heuristic(&pos);
                let this_f = (self.weights)(this_g.clone(), heuristic.clone());
                if max_cost > F::default() && this_f.clone() > max_cost {
                    continue;
                }

                match self.open.entry(pos) {
                    Occupied(entry) => {
                        let node = unsafe { &mut **entry.get() };
                        if node.t.g < this_g {
                            continue;
                        }
                        node.f = this_f.clone();
                        node.t.g = this_g.clone();
                        node.t.parent = Some(NodeRef(parent));
                        if this_f - this_g
                            < unsafe { &*best_node }.f.clone() - unsafe { &*best_node }.t.g.clone()
                        {
                            best_node = node as *const _;
                        }
                    }
                    Vacant(entry) => {
                        let node = self.arena.alloc(Node {
                            f: this_f.clone(),
                            pos,
                            t: AStarT {
                                g: this_g.clone(),
                                parent: Some(NodeRef(parent)),
                            },
                        });
                        entry.insert(node);
                        if this_f - this_g
                            < unsafe { &*best_node }.f.clone() - unsafe { &*best_node }.t.g.clone()
                        {
                            best_node = node;
                        }
                        self.heap.push(AStarOrd(node));
                    }
                }
            }
        }
        None
    }
}
