use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChunkPos {
    x: i16,
    z: i16,
}

pub struct Node<T, Pos> {
    pub pos: Pos,

    pub f: f32,
    pub t: T,
}
impl<T, Pos> Clone for Node<T, Pos>
where
    T: Clone,
    Pos: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pos: self.pos.clone(),
            f: self.f.clone(),
            t: self.t.clone(),
        }
    }
}

impl<T, Pos> PartialEq for Node<T, Pos> {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}
impl<T, Pos> Eq for Node<T, Pos> {}
impl<T, Pos> PartialOrd for Node<T, Pos> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.f.partial_cmp(&other.f)
    }
}
impl<T, Pos> Ord for Node<T, Pos> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.f > other.f {
            Ordering::Greater
        } else if self.f == other.f {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
impl<T, Pos> From<(Pos, (f32, T))> for Node<T, Pos> {
    fn from((pos, (f, t)): (Pos, (f32, T))) -> Self {
        Self { pos, f, t }
    }
}
impl<T, Pos> Into<(Pos, (f32, T))> for Node<T, Pos> {
    fn into(self) -> (Pos, (f32, T)) {
        (self.pos, (self.f, self.t))
    }
}
