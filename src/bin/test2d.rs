#![feature(test)]
#![feature(optimize_attribute)]
#![cfg_attr(feature = "jemallocator", feature(allocator_api))]

#[cfg(feature = "jemallocator")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::f32::consts::SQRT_2;

use astar::pathfinding::{self, Goal, Movements, Pathfinder, PathfinderGen};
use fixed::{types::extra::U10, FixedI32};
use fixed_sqrt::FixedSqrt;
use ordered_float::OrderedFloat;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Pos(i32, i32);
impl Goal<i32, Pos> for Pos {
    #[optimize(speed)]
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == self
    }
    #[optimize(speed)]
    fn heuristic(&self, Pos(x1, y1): &Pos) -> i32 {
        let Pos(x0, y0) = self;
        let (x, y) = ((x0 - x1), (y0 - y1));
        f32::sqrt((x * x + y * y) as f32) as i32
    }
}

// impl Goal<OrderedFloat<f64>, Pos> for Pos {
//     fn is_reached(&self, pos: &Pos) -> bool {
//         pos == self
//     }
//     fn heuristic(&self, Pos(x1, y1): &Pos) -> OrderedFloat<f64> {
//         let Pos(x0, y0) = self;
//         let (x, y) = ((x0 - x1) as f64, (y0 - y1) as f64);
//         f64::sqrt(x * x + y * y).into()
//     }
// }
// impl Goal<FixedI32<U10>, Pos> for Pos {
//     fn is_reached(&self, pos: &Pos) -> bool {
//         pos == self
//     }
//     fn heuristic(&self, Pos(x1, y1): &Pos) -> FixedI32<U10> {
//         let Pos(x0, y0) = self;
//         let (x, y) = ((x0 - x1), (y0 - y1));

//         let x = FixedI32::<U10>::from_bits(((x * x) + (y * y)) << 10);
//         // 0x0001
//         // print!("{}\t{}\t", x, y);
//         // println!("{}\t{}", (x * x), (y * y));
//         // (x + y).abs().sqrt()(x * x + y * y).sqrt()
//         FixedI32::<U10>::from_bits(x.sqrt().to_bits() as i32)
//     }
// }

impl Goal<OrderedFloat<f32>, Pos> for Pos {
    #[optimize(speed)]
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == self
    }
    #[optimize(speed)]
    fn heuristic(&self, Pos(x1, y1): &Pos) -> OrderedFloat<f32> {
        let Pos(x0, y0) = self;
        let (x, y) = ((*x0 as f32 - *x1 as f32), (*y0 as f32 - *y1 as f32));
        f32::sqrt(x * x + y * y).into()
    }
}

struct Manhattan(Pos);
impl Goal<OrderedFloat<f32>, Pos> for Manhattan {
    #[optimize(speed)]
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == &self.0
    }
    #[optimize(speed)]
    fn heuristic(&self, Pos(x1, y1): &Pos) -> OrderedFloat<f32> {
        let Pos(x0, y0) = self.0;
        // let (x, y) = ((x0 as f32 - *x1 as f32), (y0 as f32 - *y1 as f32));
        // f32::sqrt(x * x + y * y).into()
        ((x0 as f32 - *x1 as f32).abs() + (y0 as f32 - *y1 as f32).abs()).into()
    }
}
struct Huh(Pos);
impl Goal<OrderedFloat<f32>, Pos> for Huh {
    #[optimize(speed)]
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == &self.0
    }
    #[optimize(speed)]
    fn heuristic(&self, Pos(x1, y1): &Pos) -> OrderedFloat<f32> {
        let Pos(x0, y0) = self.0;
        // let (x, y) = ((x0 as f32 - *x1 as f32), (y0 as f32 - *y1 as f32));
        // f32::sqrt(x * x + y * y).into()
        ((x0 as f32 - *x1 as f32)
            .abs()
            .max((y0 as f32 - *y1 as f32).abs()))
        .into()
    }
}
struct Chebyshev(Pos);
impl Goal<OrderedFloat<f32>, Pos> for Chebyshev {
    #[optimize(speed)]
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == &self.0
    }
    #[optimize(speed)]
    fn heuristic(&self, Pos(x1, y1): &Pos) -> OrderedFloat<f32> {
        let Pos(x0, y0) = self.0;
        // let (x, y) = ((x0 as f32 - *x1 as f32), (y0 as f32 - *y1 as f32));
        // f32::sqrt(x * x + y * y).into()
        ((x0 as f32 - *x1 as f32)
            .abs()
            .max((y0 as f32 - *y1 as f32).abs()))
        .into()
    }
}
struct Octile(Pos);
impl Goal<OrderedFloat<f32>, Pos> for Octile {
    #[optimize(speed)]
    fn is_reached(&self, pos: &Pos) -> bool {
        pos == &self.0
    }
    #[optimize(speed)]
    fn heuristic(&self, Pos(x1, y1): &Pos) -> OrderedFloat<f32> {
        let Pos(x0, y0) = self.0;
        let (x, y) = ((x0 as f32 - *x1 as f32).abs(), (y0 as f32 - *y1 as f32).abs());
        // f32::sqrt(x * x + y * y).into()
        // ((x0 as f32 - *x1 as f32).abs().max((y0 as f32 - *y1 as f32).abs())).into()
        // (x.max(y) + (SQRT_2 - 1.0) * x.min(y)).into()

        if x > y {
            SQRT_2 * x + y
        } else {
            SQRT_2 * y + x
        }.into()
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
    // use pathfinding::gen_astar::AStar;
    use pathfinding::astar_slaballoc::AStar;
    const START: Pos = Pos(1500, 1999);

    /*
    a x x x x x     .
      x x x x x x   .
        x x x x x x .
          x x x x x b
    */

    const GOAL: Manhattan = Manhattan(Pos(0, 0));
    const REFPOOL_SIZE: usize = 20000;
    // let max_cost: OrderedFloat<f32> = OrderedFloat::from(f32::MAX);
    let iters = 2000;
    println!("allocating");
    let mut astar = AStar::<OrderedFloat<f32>, _>::with_refpool_size(REFPOOL_SIZE);
    println!("starting");
    let t = std::time::Instant::now();
    for _ in 0..iters {
        let result = astar.compute(START, GOAL, Adjacent);
        extern crate test;
        test::black_box(result);
        // println!("{:?}", result);
    }
    // astar.dbg();

    println!("done, took: {:?} per 100 iter", t.elapsed() * 100 / iters);
}

#[cfg(test)]
mod test {
    use super::*;
    use astar::pathfinding::{Node, Pathfinder};
    use test::Bencher;

    const ITERS: usize = 25;
    const START: Pos = Pos(0, 100);
    const GOAL: Octile = Octile(Pos(100, -100));
    const REFPOOL_SIZE: usize = 20000;

    extern crate test;

    // #[bench]
    // fn f32_fatnongeneric(b: &mut Bencher) {
    //     use pathfinding::astarf32::AStar;
    //     let mut astar: AStar<Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

    //     b.iter(|| {
    //         for _ in 0..ITERS {
    //             test::black_box(astar.compute(START, GOAL, Adjacent));
    //         }
    //     })
    // }
    // #[bench]
    // fn f32_fat(b: &mut Bencher) {
    //     use pathfinding::astar_fat::AStar;
    //     let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

    //     b.iter(|| {
    //         for _ in 0..ITERS {
    //             test::black_box(astar.compute(START, GOAL, Adjacent));
    //         }
    //     })
    // }
    #[bench]
    fn f32_rev(b: &mut Bencher) {
        use pathfinding::astar_rev::AStar;
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
            // astar.dbg();
        })
    }
    // #[bench]
    // fn f32_slaballoc(b: &mut Bencher) {
    //     use pathfinding::astar_slaballoc::AStar;
    //     let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

    //     b.iter(|| {
    //         for _ in 0..ITERS {
    //             test::black_box(astar.compute(START, GOAL, Adjacent));
    //         }
    //         // astar.dbg();
    //     })
    // }
    #[bench]
    fn f32_arena(b: &mut Bencher) {
        use pathfinding::astar_arena::{AStar, AStarT};
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_capacity(
            REFPOOL_SIZE
                * core::mem::size_of::<Node<OrderedFloat<f32>, Pos, AStarT<OrderedFloat<f32>, Pos>>>(
                ),
        );

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
            // astar.dbg();
        })
    }

    const FW: f32 = 1.5;
    const W: f32 = 2.0;
    #[bench]
    fn f32_pxwu(b: &mut Bencher) {
        use pathfinding::wastar::AStar;
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE, pxwu);

        #[optimize(speed)]
        fn pxwu(g: OrderedFloat<f32>, h: OrderedFloat<f32>) -> OrderedFloat<f32> {
            let h = h * FW;
            if g < h * (2.0 * W - 1.0) {
                g / (2.0 * W - 1.0) + h
            } else {
                (g + h) / W
            }
        }

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
            // astar.dbg();
        })
    }
    #[bench]
    fn f32_pxwd(b: &mut Bencher) {
        use pathfinding::wastar::AStar;
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE, pxwd);

        #[optimize(speed)]
        fn pxwd(g: OrderedFloat<f32>, h: OrderedFloat<f32>) -> OrderedFloat<f32> {
            let h = h * FW;
            if g < h {
                g + h
            } else {
                (g + h * (2.0 * W - 1.0)) / W
                // ( - ((g + h * (2.0 * W - 1.0)) / W).log2()
            }
        }

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
            // astar.dbg();
        })
    }
    #[bench]
    fn f32_pxwu_arena(b: &mut Bencher) {
        use pathfinding::wastar_arena::{AStar, AStarT};
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_capacity(
            REFPOOL_SIZE
                * core::mem::size_of::<Node<OrderedFloat<f32>, Pos, AStarT<OrderedFloat<f32>, Pos>>>(
                ),
            pxwu,
        );

        #[optimize(speed)]
        fn pxwu(g: OrderedFloat<f32>, h: OrderedFloat<f32>) -> OrderedFloat<f32> {
            let h = h * FW;
            if g < h * (2.0 * W - 1.0) {
                g / (2.0 * W - 1.0) + h
            } else {
                (g + h) / W
            }
        }

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
            // astar.dbg();
        })
    }
    #[bench]
    fn f32_pxwd_arena(b: &mut Bencher) {
        use pathfinding::wastar_arena::{AStar, AStarT};
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_capacity(
            REFPOOL_SIZE
                * core::mem::size_of::<Node<OrderedFloat<f32>, Pos, AStarT<OrderedFloat<f32>, Pos>>>(
                ),
            pxwd,
        );

        #[optimize(speed)]
        fn pxwd(g: OrderedFloat<f32>, h: OrderedFloat<f32>) -> OrderedFloat<f32> {
            let h = h * FW;
            if g < h {
                g + h
            } else {
                (g + h * (2.0 * W - 1.0)) / W
                // ( - ((g + h * (2.0 * W - 1.0)) / W).log2()
            }
        }

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
            // astar.dbg();
        })
    }

    // #[bench]
    // fn gen_f32(b: &mut Bencher) {
    //     use astar::pathfinding::PathfinderGen;
    //     use pathfinding::gen_astar::AStar;

    //     let MAX_COST: OrderedFloat<f32> = OrderedFloat::from(f32::MAX);
    //     let mut astar = AStar::new(REFPOOL_SIZE, MAX_COST, START, GOAL, Adjacent);

    //     b.iter(|| {
    //         for _ in 0..ITERS {
    //             test::black_box(astar.compute());
    //         }
    //     })
    // }

    #[bench]
    fn f32(b: &mut Bencher) {
        use pathfinding::astar::AStar;
        let mut astar: AStar<OrderedFloat<f32>, Pos> = AStar::with_refpool_size(REFPOOL_SIZE);

        b.iter(|| {
            for _ in 0..ITERS {
                test::black_box(astar.compute(START, GOAL, Adjacent));
            }
            // astar.dbg();
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
