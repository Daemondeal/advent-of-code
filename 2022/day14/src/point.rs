use std::{
    fmt::Display,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Point {
    pub fn normalized_int(self) -> Self {
        (self.x.signum(), self.y.signum()).into()
    }
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
