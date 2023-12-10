use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn new_from_usize(x: usize, y: usize) -> Self {
        Position { x: x as i32, y: y as i32 }
    }

    pub fn init_empty() -> Self {
        Position { x: -1, y: -1 }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
