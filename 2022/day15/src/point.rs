use std::{
    fmt::Display,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
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

    pub fn taxicab_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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

impl From<(i64, i64)> for Point {
    fn from(other: (i64, i64)) -> Self {
        Self {
            x: other.0,
            y: other.1,
        }
    }
}

impl From<Point> for (i64, i64) {
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
