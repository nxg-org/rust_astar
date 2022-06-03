mod astar;
mod node;
mod chunk_data;


mod debug {
    use super::astar::{AStar, Heuristic};


    pub struct Position {
        pub x: i32,
        pub y: i32
    }

    impl Heuristic<Position> for Position {
        fn is_goal(&self, p: &Position) -> bool {
        todo!()
    }

        fn heuristic(&self, p: &Position) -> f32 {
        todo!()
    }
    }

    #[test]
    fn astar_test() {

        let start = Position { x: 0, y: 0 };

    //     let astar = AStar::new(start, );
    }
}