use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{fmt::Debug, hash::Hash};

use ordered_float::OrderedFloat;

use super::{Goal, Movements, Node, Pathfinder};

#[derive(Clone, Debug)]
pub struct AStarT<Pos> {
    g: OrderedFloat<f32>,
    h: OrderedFloat<f32>,
    parent: Option<refpool::PoolRef<NodeLeaf<Pos>>>,
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

pub struct AStar<Pos> {
    closed: HashSet<Pos>,
    heap: BinaryHeap<Node<OrderedFloat<f32>, Pos, AStarT<Pos>>>,
    open: HashMap<Pos, (OrderedFloat<f32>, AStarT<Pos>)>,
    refpool: refpool::Pool<NodeLeaf<Pos>>,
}

impl<Pos> AStar<Pos> {
    pub fn with_refpool_size(size: usize) -> Self {
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
    fn path(&mut self, end_node: Node<OrderedFloat<f32>, Pos, AStarT<Pos>>) -> Vec<Pos>
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

impl<Pos> Pathfinder for AStar<Pos>
where
    Pos: Hash + Eq + Clone + Copy,
{
    type F = OrderedFloat<f32>;
    type Pos = Pos;

    fn compute(
        &mut self,
        start: Self::Pos,
        goal: impl Goal<OrderedFloat<f32>, Pos>,
        movements: impl Movements<OrderedFloat<f32>, Pos>,
    ) -> Option<Vec<Self::Pos>> {
        self.clear();

        let h = goal.heuristic(&start);
        let start_node = Node {
            f: h,
            pos: start,
            t: AStarT {
                g: 0.0.into(),
                h,
                parent: None,
            },
        };
        self.heap.push(start_node.clone());

        let max_cost: OrderedFloat<f32> = 0.0.into();
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

                let this_g = node.t.g + cost;

                let heuristic = goal.heuristic(&neighbor_pos);
                if max_cost > 0.0.into() && this_g + heuristic > max_cost {
                    continue;
                }

                match self.open.entry(neighbor_pos) {
                    Occupied(mut entry) => {
                        if entry.get().1.g < this_g {
                            continue;
                        }
                        let node = entry.get_mut();
                        let (f, t) = node;
                        *f = this_g + heuristic;
                        t.g = this_g;
                        t.h = heuristic;
                        t.parent = Some(parent.clone());
                        if heuristic < best_node.t.h {
                            best_node = (neighbor_pos, (*f, t.clone())).into();
                        }
                    }
                    Vacant(entry) => {
                        let node = entry.insert((
                            this_g + heuristic,
                            AStarT {
                                g: this_g,
                                h: heuristic,
                                parent: Some(parent.clone()),
                            },
                        ));
                        let neighbor = (neighbor_pos, node.clone()).into();
                        if heuristic < best_node.t.h {
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
