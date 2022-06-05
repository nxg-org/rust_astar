use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{fmt::Debug, hash::Hash};

use refpool::{Pool, PoolBox, PoolRef};

use super::{Goal, Movements, Node, Pathfinder};

#[derive(Clone, Debug)]
pub struct AStarT<F, Pos> {
    g: F,
    parent: Option<NodeRef<F, Pos>>,
}

#[derive(Debug)]
pub struct NodeRef<F, Pos>(PoolRef<Node<F, Pos, AStarT<F, Pos>>>);
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

pub struct AStar<F, Pos> {
    refpool: Pool<Node<F, Pos, AStarT<F, Pos>>>,
    heap: BinaryHeap<PoolBox<Node<F, Pos, AStarT<F, Pos>>>>,
    open: HashMap<Pos, *mut Node<F, Pos, AStarT<F, Pos>>>,
    closed: HashSet<Pos>,
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
    pub fn with_refpool_size(size: usize) -> Self
    where
        F: Ord,
    {
        Self {
            closed: Default::default(),
            heap: Default::default(),
            open: Default::default(),
            refpool: refpool::Pool::new(size),
        }
    }
    fn clear(&mut self) {
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
            v.push(p.0.pos.clone());
            pp = p.0.t.parent.clone();
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
        let start_node = PoolBox::new(
            &self.refpool,
            Node {
                f: h.clone(),
                pos: start,
                t: AStarT {
                    g: F::default(),
                    parent: None,
                },
            },
        );
        let max_cost = F::default();
        let mut best_node: *const Node<F, Pos, AStarT<F, Pos>> = &*start_node as _;
        self.heap.push(start_node);

        while let Some(node) = self.heap.pop() {
            if goal.is_reached(&node.pos) {
                // self.dbg();
                return Some(node.path());
            }
            self.open.remove(&node.pos);
            self.closed.insert(node.pos);

            let neighbors = movements.get_neighbors(node.pos);
            let parent = unsafe { PoolRef::from_raw(PoolBox::into_raw(node)) };
            for (pos, cost) in neighbors {
                if self.closed.contains(&pos) {
                    continue;
                }

                let g = parent.t.g.clone() + cost;
                let heuristic = goal.heuristic(&pos);
                if max_cost > F::default() && g.clone() + heuristic.clone() > max_cost {
                    continue;
                }

                match self.open.entry(pos) {
                    Occupied(mut entry) => {
                        let node = entry.get();
                        if unsafe { &**node }.t.g < g {
                            continue;
                        }
                        let node = unsafe { &mut **node };
                        node.f = g.clone() + heuristic.clone();
                        node.t.g = g;
                        node.t.parent = Some(NodeRef(parent.clone()));
                        if heuristic
                            < unsafe { &*best_node }.f.clone() - unsafe { &*best_node }.t.g.clone()
                        {
                            best_node = node as *const _;
                        }
                    }
                    Vacant(entry) => {
                        let mut node = PoolBox::new(
                            &self.refpool,
                            Node {
                                f: g.clone() + heuristic.clone(),
                                pos,
                                t: AStarT {
                                    g,
                                    parent: Some(NodeRef(parent.clone())),
                                },
                            },
                        );
                        entry.insert(node.as_mut());
                        if heuristic
                            < unsafe { &*best_node }.f.clone() - unsafe { &*best_node }.t.g.clone()
                        {
                            best_node = node.as_ref() as *const _;
                        }
                        self.heap.push(node);
                    }
                }
            }
        }
        None
    }
}
