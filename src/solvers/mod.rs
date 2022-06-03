pub mod astar;
mod generic_goal;
mod generic_solver;
mod path_types;

pub mod debug {
    use std::rc::Rc;

    use crate::{
        path_node::Node,
        world_info::{Block, Grid}, solvers::path_types::PathResult,
    };

    use super::{astar::AStarSolver, generic_goal::GoalNode, generic_solver::PathSolver};

    #[test]
    fn gen_astar_testing() {
        let far_x = 2;
        let far_y = 3;
        // we want to generate the nodes, ig
        // already did dw, look at this
        let raw_grid: Vec<Block> = [
            (-2, -3, 0),
            (-1, -3, 0),
            (0, -3, 1),
            (1, -3, 0),
            (2, -3, 0),
            (-2, -2, 0),
            (-1, -2, 0),
            (0, -2, 1),
            (1, -2, 0),
            (2, -2, 0),
            (-2, -1, 0),
            (-1, -1, 0),
            (0, -1, 1),
            (1, -1, 0),
            (2, -1, 0),
            (-2, 0, 0),
            (-1, 0, 0),
            (0, 0, 0),
            (1, 0, 0),
            (2, 0, 0),
            (-2, 1, 0),
            (-1, 1, 0),
            (0, 1, 1),
            (1, 1, 0),
            (2, 1, 0),
            (-2, 2, 0),
            (-1, 2, 0),
            (0, 2, 1),
            (1, 2, 0),
            (2, 2, 0),
            (-2, 3, 0),
            (-1, 3, 0),
            (0, 3, 1),
            (1, 3, 0),
            (2, 3, 0),
        ]
        .into_iter()
        .map(|val| val.into())
        .collect();

        // with this, you can specified X specific nodes while all others are set to default, MAX cost.
        let grid = Grid::with_unchecked_pos_infos(&raw_grid, far_x, far_y);


        let start_node = Node::zero_cost(-2, -3);
        let goal = GoalNode(Node::zero_cost(2, 3));
        let mut astar = AStarSolver::new(
            start_node,
            goal,
            |first, second| {
                ((first.data.x - second.data.x).pow(2) + (first.data.x - second.data.y).pow(2))
                    as f32
            },
            1000.0,
            10 * 1000,
            grid,
        );

        let path = astar.calculate();

        println!("{:?}", path);

        match path {

            PathResult::Complete(raw) => {
                
            }
            _ => {}
        }


    }
}
