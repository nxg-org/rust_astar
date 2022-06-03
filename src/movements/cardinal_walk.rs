use super::generic_movement::{GenericMovementInfo, GenericMovement};

pub const CARDINAL_POSITIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (1, 0), (-1, 0)];


pub struct CardinalWalk {
    pub info: GenericMovementInfo
}


impl GenericMovement for CardinalWalk {
    fn calculate_cost(&self) -> f32 {
        todo!()
    }

    fn in_valid_position(&self) -> bool {
        todo!()
    }

    fn perform_movement(&self) {
        todo!()
    }
}