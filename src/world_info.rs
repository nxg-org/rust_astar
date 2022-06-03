use std::hash::Hash;

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    pub inner: Vec<Block>,
}

impl Grid {
    pub fn get_bounds(&self) -> Bounds {
        Bounds::new(self.width / 2, self.height / 2)
    }

    // pub fn get_pos_info_mut(&mut self, x: i32, y: i32) -> Option<&mut PosInfo> {
    //     self.inner.get_mut(self.get_bounds().get_index(x, y))
    // }
    pub fn get_pos_info_ref(&self, x: i32, y: i32) -> Option<&Block> {
        self.inner.get(self.get_bounds().get_index(x, y))
    }
    pub fn get_pos_info(&self, x: i32, y: i32) -> Option<Block> {
        self.inner
            .get(self.get_bounds().get_index(x, y))
            .map(Clone::clone)
    }

    // this can return MAX (ye, if it does then its invalid,
    // it's safe to function that way as well since you
    // can't ever have index of USIZEMAX)

    pub fn with_unchecked_pos_infos(
        pos_infos: &[Block],
        half_width: usize,
        half_height: usize,
    ) -> Self {
        let bounds = Bounds::new(half_width, half_height);
        let max = (half_width * 2 + 1) * (half_height * 2 + 1);
        let mut final_pos_infos = Vec::with_capacity(max);
        for y in -(half_height as i32)..=(half_height as i32) {
            for x in -(half_width as i32)..=(half_width as i32) {
                final_pos_infos.push(Block::impassable(x, y))
            }
        }
        let mut ind: usize;
        for pos_info in pos_infos {
            ind = bounds.get_index(pos_info.x, pos_info.y);

            // following accounts for usize::MAX lol, so no worries
            // (also elegant since index of array must be usize::MAX - 1, so usize::MAX can signify not found).
            if ind < max {
                final_pos_infos[ind] = *pos_info;
            }
        }

        Self {
            width: half_width * 2 + 1,
            height: half_height * 2 + 1,
            inner: final_pos_infos,
        }
    }

    pub fn with_full_pos_infos(pos_infos: &[Block], width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            inner: pos_infos.to_vec(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Bounds {
    width: usize,
    height: usize,
    half_width: usize,
    half_height: usize,
    max: usize,
}

/// format is: (width, height, actual index)
/// Grid right now is { half_width: 2, half_height: 3}
/// [(-2, -3, 0),  (-1, -3, 1),  (0, -3, 2),  (1, -3, 3),  (2, -3, 4),
///  (-2, -2, 5),  (-1, -2, 6),  (0, -2, 7),  (1, -2, 8),  (2, -2, 9),
///  (-2, -1, 10), (-1, -1, 11), (0, -1, 12), (1, -1, 13), (2, -1, 14),
///  (-2, 0, 15),  (-1, 0, 16),  (0, 0, 17),  (1, 0, 18),  (2, 0, 19),
///  (-2, 1, 20),  (-1, 1, 21),  (0, 1, 22),  (1, 1, 23),  (2, 1, 24),
///  (-2, 2, 25),  (-1, 2, 26),  (0, 2, 27),  (1, 2, 28),  (2, 2, 29),
///  (-2, 3, 30),  (-1, 3, 31),  (0, 3, 32),  (1, 3, 33),  (2, 3, 34)]
impl Bounds {
    pub fn new(half_width: usize, half_height: usize) -> Self {
        Self {
            width: half_width * 2 + 1,
            height: half_height * 2 + 1,
            half_width,
            half_height,
            max: (half_width * 2 + 1) * (half_height * 2 + 1),
        }
    }

    #[inline]
    pub fn is_in_bounds(self, x: i32, y: i32) -> bool {
        self.get_index(x, y) != usize::MAX
    }

    /// Examples:
    /// (-2, -3, 0): ((-3 + 3) * 5) + (2 +(-2)) = 0
    /// (-2, 2, 25): ((2 + 3) * 5) + (2 + (-2)) = 25
    /// (1, 3, 33): ((3 + 3) * 5) + (2 + 1) = 33
    #[inline]
    pub fn get_index(self, x: i32, y: i32) -> usize {
        let tmp = x + self.half_width as i32;

        let (_val, overflowed) = self.width.overflowing_sub(tmp as usize);
        if overflowed {
            return usize::MAX;
        }

        let tmp1 = y + self.half_height as i32;
        let (_val1, overflowed1) = self.height.overflowing_sub(tmp1 as usize);
        if overflowed1 {
            return usize::MAX;
        }

        (tmp1 * self.width as i32 + tmp) as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub passable: bool,
    pub terrain_speed: f32,
}

impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Block {}

impl Block {
    #[inline]
    pub fn string_hash(&self) -> String {
        format!("{},{}", self.x, self.y)
    }

    pub fn impassable(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            passable: false,
            terrain_speed: 0f32,
        }
    }

    pub fn passable(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            passable: true,
            terrain_speed: 1f32,
        }
    }
}

impl From<(i32, i32, i32)> for Block {
    fn from(val: (i32, i32, i32)) -> Self {
        Self {
            x: val.0,
            y: val.1,
            passable: val.2 != 0,
            terrain_speed: val.2 as f32,
        }
    }
}

impl From<(i32, i32, f32)> for Block {
    fn from(val: (i32, i32, f32)) -> Self {
        Self {
            x: val.0,
            y: val.1,
            passable: val.2 != 0f32,
            terrain_speed: val.2,
        }
    }
}
