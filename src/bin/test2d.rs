#![feature(test)]

use std::f32::consts::SQRT_2;

use astar::pathfinding::{self, Goal, Movements, Pathfinder};
use fixed::{types::extra::U10, FixedI32};
use fixed_sqrt::FixedSqrt;
use ordered_float::OrderedFloat;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Pos(i32, i32);
impl Goal<i32, Pos> for Pos {
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == self
    }
    fn heuristic(&self, Pos(x1, y1): &Pos) -> i32 {
        let Pos(x0, y0) = self;
        let (x, y) = ((x0 - x1), (y0 - y1));
        f32::sqrt((x * x + y * y) as f32) as i32
    }
}

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
impl Goal<OrderedFloat<f64>, Pos> for Pos {
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == self
    }
    fn heuristic(&self, Pos(x1, y1): &Pos) -> OrderedFloat<f64> {
        let Pos(x0, y0) = self;
        let (x, y) = ((x0 - x1) as f64, (y0 - y1) as f64);
        f64::sqrt(x * x + y * y).into()
    }
}
impl Goal<FixedI32<U10>, Pos> for Pos {
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == self
    }
    fn heuristic(&self, Pos(x1, y1): &Pos) -> FixedI32<U10> {
        let Pos(x0, y0) = self;
        let (x, y) = ((x0 - x1), (y0 - y1));

        let x = FixedI32::<U10>::from_bits(((x * x) + (y * y)) << 10);
        // 0x0001
        // print!("{}\t{}\t", x, y);
        // println!("{}\t{}", (x * x), (y * y));
        // (x + y).abs().sqrt()(x * x + y * y).sqrt()
        FixedI32::<U10>::from_bits(x.sqrt().to_bits() as i32)
    }
}

// #[test]
// fn a() {
//     println!(
//         "{:x?}",
//         FixedI32::<U10>::from_bits(0b1 << 17)
//             .sqrt()
//             .to_bits()
//             .to_be_bytes()
//     )
// }

struct Adjacent;
// impl Movements<i32, Pos> for Adjacent {
//     fn get_neighbors(&self, pos: Pos) -> Vec<(Pos, i32)> {
//         vec![
//             (Pos(pos.0, pos.1 + 1), 1000),
//             (Pos(pos.0, pos.1 - 1), 1000),
//             (Pos(pos.0 + 1, pos.1), 1000),
//             (Pos(pos.0 - 1, pos.1), 1000),
//             (Pos(pos.0, pos.1 + 1), 1414),
//             (Pos(pos.0, pos.1 - 1), 1414),
//             (Pos(pos.0 + 1, pos.1), 1414),
//             (Pos(pos.0 - 1, pos.1), 1414),
//         ]
//     }
// }
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
impl Movements<OrderedFloat<f64>, Pos> for Adjacent {
    fn get_neighbors(&self, pos: Pos) -> Vec<(Pos, OrderedFloat<f64>)> {
        use core::f64::consts::SQRT_2;
        vec![
            (Pos(pos.0, pos.1 + 1), 1.0.into()),
            (Pos(pos.0, pos.1 - 1), 1.0.into()),
            (Pos(pos.0 + 1, pos.1), 1.0.into()),
            (Pos(pos.0 - 1, pos.1), 1.0.into()),
            (Pos(pos.0 + 1, pos.1 + 1), SQRT_2.into()),
            (Pos(pos.0 + 1, pos.1 - 1), SQRT_2.into()),
            (Pos(pos.0 - 1, pos.1 + 1), SQRT_2.into()),
            (Pos(pos.0 - 1, pos.1 - 1), SQRT_2.into()),
        ]
    }
}
const FIXED_SQRT: FixedI32<U10> = FixedI32::from_bits(0x16a00 >> 6); // >> 1
const FIXED_ONE: FixedI32<U10> = FixedI32::from_bits(0x10000 >> 6); // >> 1

impl Movements<FixedI32<U10>, Pos> for Adjacent {
    fn get_neighbors(&self, pos: Pos) -> Vec<(Pos, FixedI32<U10>)> {
        vec![
            (Pos(pos.0, pos.1 + 1), FIXED_ONE),
            (Pos(pos.0, pos.1 - 1), FIXED_ONE),
            (Pos(pos.0 + 1, pos.1), FIXED_ONE),
            (Pos(pos.0 - 1, pos.1), FIXED_ONE),
            (Pos(pos.0 + 1, pos.1 + 1), FIXED_SQRT),
            (Pos(pos.0 + 1, pos.1 - 1), FIXED_SQRT),
            (Pos(pos.0 - 1, pos.1 + 1), FIXED_SQRT),
            (Pos(pos.0 - 1, pos.1 - 1), FIXED_SQRT),
        ]
    }
}

fn main() {
    use pathfinding::astarf32::AStar;
    let mut astar: AStar<Pos> = AStar::with_refpool_size(1000);

    let result = Pathfinder::compute(&mut astar, Pos(0, 0), Pos(4, 9), Adjacent);

    println!("{:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;
    use astar::pathfinding::Pathfinder;
    use test::Bencher;

    const ITERS: usize = 100;
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
    fn f32_lateheuristic(b: &mut Bencher) {
        use pathfinding::astarheuristiclater::AStar;
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
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
    #[bench]
    fn f32_generic(b: &mut Bencher) {
        use pathfinding::astar::AStar;
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
        })
    }
    #[bench]
    fn f64_generic(b: &mut Bencher) {
        use pathfinding::astar::AStar;
        let mut astar: AStar<OrderedFloat<f64>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
        })
    }
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
    #[bench]
    fn fixed(b: &mut Bencher) {
        use pathfinding::astarfixed::AStar;
        let mut astar = AStar::with_refpool_size(REFPOOL_SIZE);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
        })
    }
    #[bench]
    fn fixed_generic(b: &mut Bencher) {
        use pathfinding::astar::AStar;
        let mut astar: AStar<FixedI32<U10>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
        })
    }
}
