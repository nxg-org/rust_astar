// pub mod astari32;
pub mod astarf32;
pub mod astar;
pub mod astarthin;
pub mod gen_astar;

/// Represents a node in the pathfinding algorithms
///
/// F can be anything which can be compared, usually f32 or f64
/// The Ord of Node is the opposite compared to the implementation
/// of Ord on F, this allows for a Min-Heap in AStar and more
///
/// T is whatever data an algorithm additionally wants to store
/// in each node
#[derive(Clone, Debug)]
pub struct Node<F, Pos, T> {
    pub(crate) f: F,
    pub(crate) pos: Pos,
    pub(crate) t: T,
}

impl<F, Pos, T> PartialEq for Node<F, Pos, T>
where
    F: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}
impl<F, Pos, T> Eq for Node<F, Pos, T> where F: Eq {}
impl<F, Pos, T> PartialOrd for Node<F, Pos, T>
where
    F: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f.partial_cmp(&self.f)
    }
}
impl<F, Pos, T> Ord for Node<F, Pos, T>
where
    F: Ord
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl<F, Pos, T> From<(Pos, (F, T))> for Node<F, Pos, T> {
    fn from((pos, (f, t)): (Pos, (F, T))) -> Self {
        Self { pos, f, t }
    }
}
impl<F, Pos, T> From<Node<F, Pos, T>> for (Pos, (F, T)) {
    fn from(val: Node<F, Pos, T>) -> Self {
        (val.pos, (val.f, val.t))
    }
}

pub trait Goal<F, Pos> {
    fn is_reached(&self, pos: &Pos) -> bool;
    fn heuristic(&self, pos: &Pos) -> F;
}
pub trait Movements<F, Pos> {
    fn get_neighbors(&self, pos: Pos) -> Vec<(Pos, F)>;
}

pub trait Pathfinder {
    type F;
    type Pos;
    fn compute(
        &mut self,
        start: Self::Pos,
        goal: impl Goal<Self::F, Self::Pos>,
        movements: impl Movements<Self::F, Self::Pos>,
    ) -> Option<Vec<Self::Pos>>;
}


pub trait PathfinderGen {
    type F;
    type Pos;
    type Movements;
    type Goal;

    fn new(
        size: usize,
        max_cost: Self::F,
        start: Self::Pos,
        goal: Self::Goal,
        movements: Self::Movements,
    ) -> Self;

    fn compute(&mut self) -> PathResult<Vec<Self::Pos>>;

    fn reset(&mut self);

    // fn reset(&mut self);
}

#[derive(Debug)]
pub enum PathResult<T> {
    Partial(T),
    Complete(T),
    Timeout(T),
    NoPath(T),
}
