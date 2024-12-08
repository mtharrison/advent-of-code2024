#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2d {
    pub x: i32,
    pub y: i32,
}

impl Vec2d {
    pub fn new(x: i32, y: i32) -> Vec2d {
        Vec2d { x, y }
    }

    fn add_internal(&self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub_internal(&self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        self.add_internal(other)
    }
}

impl std::ops::Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        self.sub_internal(other)
    }
}
