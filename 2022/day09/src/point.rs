use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(other: (i32, i32)) -> Self {
        Self {
            x: other.0,
            y: other.1,
        }
    }
}

impl From<Point> for (i32, i32) {
    fn from(other: Point) -> Self {
        (other.x, other.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
