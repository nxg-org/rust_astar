#[derive(Clone, Copy)]
pub struct Block(u64);
impl Block {
    pub fn is_solid(self) -> bool {
        self.0 > 0
    }
}

pub trait ChunkDataProvider {
    fn get_block(&self, x: i32, y: i32, z: i32) -> Block;
}

impl<T> ChunkDataProvider for T
where
    T: Fn(i32, i32, i32) -> Block,
{
    fn get_block(&self, x: i32, y: i32, z: i32) -> Block {
        self(x, y, z)
    }
}

pub fn plane(x: i32, y: i32, z: i32) -> Block {
    if y > 60 {
        Block(0)
    } else {
        Block(1)
    }
}
