#![feature(test)]

use std::f32::consts::SQRT_2;

use astar::pathfinding::{self, Goal, Movements, Pathfinder, PathfinderGen};
use fixed::{types::extra::U10, FixedI32};
use fixed_sqrt::FixedSqrt;
use ordered_float::OrderedFloat;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Pos(i32, i32);

impl Goal<OrderedFloat<f32>, Pos> for Pos {
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == self
    }
    fn heuristic(&self, Pos(x1, y1): &Pos) -> OrderedFloat<f32> {
        let Pos(x0, y0) = self;
        let (x, y) = ((*x0 as f32 - *x1 as f32), (*y0 as f32 - *y1 as f32));
        f32::sqrt(x * x + y * y).into()
    }
}


struct Adjacent;

impl Movements<OrderedFloat<f32>, Pos> for Adjacent {
    fn get_neighbors(&self, pos: Pos) -> Vec<(Pos, OrderedFloat<f32>)> {
        vec![
            (Pos(pos.0, pos.1 + 1), (1.0).into()),
            (Pos(pos.0, pos.1 - 1), (1.0).into()),
            (Pos(pos.0 + 1, pos.1), (1.0).into()),
            (Pos(pos.0 - 1, pos.1), (1.0).into()),
            (Pos(pos.0 + 1, pos.1 + 1), (SQRT_2).into()),
            (Pos(pos.0 + 1, pos.1 - 1), (SQRT_2).into()),
            (Pos(pos.0 - 1, pos.1 + 1), (SQRT_2).into()),
            (Pos(pos.0 - 1, pos.1 - 1), (SQRT_2).into()),
        ]
    }
}


struct World;

impl Movements<OrderedFloat<f32>, Pos> for World {
    fn get_neighbors(&self, pos: Pos) -> Vec<(Pos, OrderedFloat<f32>)> {

        const CARDINAL: [(i32, i32);4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        const DIAGONAL: [(i32, i32);4] = [(1, 1), (-1, 1), (-1, 1), (-1, -1)];


        let mut tmp = Vec::with_capacity(8);
        for dir in CARDINAL.iter() {
            let res: OrderedFloat<f32> = if pos.1 == 5  && pos.0 == 5  { (f32::MAX).into() } else { (1.0).into() };
            tmp.push((Pos(pos.0 + dir.0, pos.1 + dir.1), res ));
        }

        for dir in DIAGONAL.iter() {
            let res: OrderedFloat<f32> = if pos.1 == 5  && pos.0 == 5 { (f32::MAX).into() } else { (SQRT_2).into() };
            tmp.push((Pos(pos.0 + dir.0, pos.1 + dir.1), res ));
        }
        tmp
    }
}



fn main() {
    use pathfinding::gen_astar::AStar;

    const START: Pos = Pos(-10, -10);
    const GOAL: Pos = Pos(10, 10);
    const REFPOOL_SIZE: usize = 2000;
    let MAX_COST: OrderedFloat<f32> = OrderedFloat::from(100.0);
    
    let mut astar = AStar::new(REFPOOL_SIZE, MAX_COST, START, GOAL, World);
    let result = astar.compute();

    println!("{:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;
    use astar::pathfinding::{Pathfinder, PathfinderGen};
    use test::Bencher;

    const ITERS: usize = 1000;
    const START: Pos = Pos(-100, -100);
    const GOAL: Pos = Pos(100, 100);
    const REFPOOL_SIZE: usize = 2000;

    extern crate test;

    #[bench]
    fn f32(b: &mut Bencher) {
        use pathfinding::astarf32::AStar;
        let mut astar: AStar<Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
        })
    }

    #[bench]
    fn gen_f32(b: &mut Bencher) {
        use pathfinding::gen_astar::AStar;
        
        let MAX_COST: OrderedFloat<f32> = OrderedFloat::from(f32::MAX);
        let mut astar = AStar::new(REFPOOL_SIZE, MAX_COST, START, GOAL, Adjacent);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute());
            }
        })
    }

    #[bench]
    fn f32_thin(b: &mut Bencher) {
        use pathfinding::astarthin::AStar;
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
        })
    }
    // #[bench]
    // fn f32_generic(b: &mut Bencher) {
    //     use pathfinding::astar::AStar;
    //     let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

    //     b.iter(|| {
    //         for _ in 0..ITERS {
    //             test::black_box(astar.compute(START, GOAL, Adjacent));
    //         }
    //     })
    // }
    // #[bench]
    // fn f64_generic(b: &mut Bencher) {
    //     use pathfinding::astar::AStar;
    //     let mut astar: AStar<OrderedFloat<f64>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

    //     b.iter(|| {
    //         for _ in 0..ITERS {
    //             test::black_box(astar.compute(START, GOAL, Adjacent));
    //         }
    //     })
    // }
    // #[bench]
    // fn i32(b: &mut Bencher) {
    //     use pathfinding::astari32::AStar;
    //     let mut astar = AStar::with_refpool_size(1000);

    //     b.iter(|| {
    //         for _ in 0..ITERS {
    //             test::black_box(astar.compute(START, GOAL, Adjacent));
    //         }
    //     })
    // }
}
