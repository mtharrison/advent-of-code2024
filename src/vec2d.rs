use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2d<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn new(x: T, y: T) -> Vec2d<T> {
        Vec2d { x, y }
    }

    fn add_internal(&self, other: Vec2d<T>) -> Vec2d<T> {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub_internal(&self, other: Vec2d<T>) -> Vec2d<T> {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Add for Vec2d<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Vec2d<T>;

    fn add(self, other: Vec2d<T>) -> Vec2d<T> {
        self.add_internal(other)
    }
}

impl<T> Sub for Vec2d<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Vec2d<T>;

    fn sub(self, other: Vec2d<T>) -> Vec2d<T> {
        self.sub_internal(other)
    }
}
