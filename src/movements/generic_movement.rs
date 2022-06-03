
pub struct GenericMovementInfo {
    pub is_parkour: bool
}


pub trait GenericMovement {
    fn calculate_cost(&self) -> f32;

    fn in_valid_position(&self) -> bool;

    fn perform_movement(&self);
}