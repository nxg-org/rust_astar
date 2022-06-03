// Astar Relevant.
pub const CARDINAL_POSITIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
pub const DIAGONAL_POSITIONS: [(i32, i32); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
pub const ALL_OFFSETS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

// Cost relevant.
pub const MAX_COST: i32 = 10000 * 1000;
pub const SQRT_FIXED: i32 = 1414;
