use astar::pathfinding::{astar::AStar, Goal, Movements, Pathfinder};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Pos(i32, i32);
impl Goal<i32, Pos> for Pos {
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == self
    }
    fn heuristic(&self, pos: &Pos) -> i32 {
        (((pos.0 + self.0) * (pos.0 + self.0) + (pos.1 + self.1) * (pos.1 + self.1)) as f32).sqrt()
            as i32
    }
}

struct Adjacent;
impl Movements<i32, Pos> for Adjacent {
    fn get_neighbors(&self, pos: Pos) -> Vec<(Pos, i32)> {
        vec![
            (Pos(pos.0, pos.1 + 1), 1000),
            (Pos(pos.0, pos.1 - 1), 1000),
            (Pos(pos.0 + 1, pos.1), 1000),
            (Pos(pos.0 - 1, pos.1), 1000),
        ]
    }
}

fn main() {
    let mut astar = AStar::with_refpool_size(1000);

    let result = Pathfinder::compute(&mut astar, Pos(2, 4), Pos(4, 9), Adjacent);

    println!("{:?}", result);
}
