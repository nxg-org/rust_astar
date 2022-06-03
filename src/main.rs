#![feature(int_roundings)]

mod solvers;
mod path_node;
mod world_info;
use world_info::{Block, Grid, Bounds};

mod rob;

fn main() {
    let far_x = 2;
    let far_y = 3;
    // we want to generate the nodes, ig
    // already did dw, look at this
    let raw_grid: Vec<Block> =  [   (-2, -3, 0),  (-1, -3, 0),  (0, -3, 1),  (1, -3, 0),  (2, -3, 0),
                                    (-2, -2, 0),  (-1, -2, 0),  (0, -2, 1),  (1, -2, 0),  (2, -2, 0),
                                    (-2, -1, 0),  (-1, -1, 0),  (0, -1, 1),  (1, -1, 0),  (2, -1, 0),
                                    (-2, 0, 0),   (-1, 0, 0),   (0, 0, 0),   (1, 0, 0),   (2, 0, 0),
                                    (-2, 1, 0),   (-1, 1, 0),   (0, 1, 1),   (1, 1, 0),   (2, 1, 0),
                                    (-2, 2, 0),   (-1, 2, 0),   (0, 2, 1),   (1, 2, 0),   (2, 2, 0),
                                    (-2, 3, 0),   (-1, 3, 0),   (0, 3, 1),   (1, 3, 0),   (2, 3, 0)
                                ].into_iter().map(|val| val.into()).collect();


    // with this, you can specified X specific nodes while all others are set to default, MAX cost.
    let grid = Grid::with_unchecked_pos_infos(&raw_grid, far_x, far_y);

    let bounds = Bounds::new(far_x, far_y);

    // println!("{:?}", &grid);

    for item in grid.inner.into_iter() {
        println!("{:?} {}", item, bounds.get_index(item.x, item.y));
    }
    // new solver ig
}

