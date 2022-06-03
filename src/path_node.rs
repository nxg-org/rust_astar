use std::{cmp::Ordering, rc::Rc};

use crate::world_info::Block;

// we **definitely** should use copy
#[derive(Debug, Clone)]
pub struct Node {
    pub data: Block,

    /// distance from start
    pub g: f32,
    /// distance from goal
    pub h: f32,
    // g+h+heuristic
    pub f: f32,

    pub parent: Option<Rc<Node>>,
}

impl<'a> PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl<'a> Eq for Node {}
impl<'a> PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.f.partial_cmp(&other.f)
    }
}
impl<'a> Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.f > other.f {
            Ordering::Less
        } else if self.f == other.f {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl<'a> Node {
    pub fn default_max_cost() -> Self {
        Self {
            data: Block::impassable(0, 0),
            g: f32::MAX,
            h: f32::MAX,
            f: f32::MAX,
            parent: None,
        }
    }

    pub fn zero_cost(x: i32, y: i32) -> Self {
        Self {
            data: Block::passable(x, y),
            g: 0f32,
            h: 0f32,
            f: 0f32,
            parent: None,
        }
    }

    pub fn max_cost(x: i32, y: i32) -> Self {
        Self {
            data: Block::impassable(x, y),
            g: f32::MAX,
            h: f32::MAX,
            f: f32::MAX,
            parent: None,
        }
    }

    pub fn with_costs_and_data(data: Block, g: f32, h: f32, parent: Option<Rc<Node>>) -> Self {
        Self {
            data,
            g,
            h,
            f: g + h,
            parent
        }
    }

    #[inline]
    pub fn zero_costs(&mut self) {
        self.g = 0f32;
        self.h = 0f32;
        self.f = 0f32;
    }

    #[inline]
    pub fn set_costs(&mut self, g: f32, h: f32) {
        self.g = g;
        self.h = h;
        self.f = g + h;
    }

    #[inline]
    pub fn get_x(&self) -> i32 {
        self.data.x
    }

    #[inline]
    pub fn get_y(&self) -> i32 {
        self.data.y
    }
}

impl<'a> From<(i32, i32, i32, i32)> for Node {
    fn from(val: (i32, i32, i32, i32)) -> Self {
        Self {
            data: Block::passable(val.0, val.1),
            g: val.2 as f32,
            h: val.3 as f32,
            f: val.2 as f32 + val.3 as f32,
            parent: None,
        }
    }
}

impl<'a> From<(i32, i32, f32, f32)> for Node {
    fn from(val: (i32, i32, f32, f32)) -> Self {
        Self {
            data: Block::passable(val.0, val.1),
            g: val.2,
            h: val.3,
            f: val.2 + val.3,
            parent: None,
        }
    }
}

impl<'a> From<(i32, i32, i32)> for Node {
    fn from(val: (i32, i32, i32)) -> Self {
        Self {
            data: Block::passable(val.0, val.1),
            g: 0f32,
            h: 0f32,
            f: val.2 as f32,
            parent: None,
        }
    }
}

impl<'a> From<(i32, i32, f32)> for Node {
    fn from(val: (i32, i32, f32)) -> Self {
        Self {
            data: Block::passable(val.0, val.1),
            g: 0f32,
            h: 0f32,
            f: val.2 as f32,
            parent: None,
        }
    }
}

impl<'a> From<(i32, i32)> for Node {
    fn from(val: (i32, i32)) -> Self {
        Self {
            data: Block::passable(val.0, val.1),
            g: 0f32,
            h: 0f32,
            f: 0f32,
            parent: None,
        }
    }
}


impl <'a> Into<Block> for Node {
    fn into(self) -> Block {
        self.data
    }
}
 

impl <'a> Into<Block> for &Node {
    fn into(self) -> Block {
        self.data
    }
}
 