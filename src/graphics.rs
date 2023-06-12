use super::*;

use std::ops::{Add, Sub};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
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

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: if rhs.x < self.x { self.x - rhs.x } else { 0 },
            y: if rhs.y < self.y { self.y - rhs.y } else { 0 },
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0.try_into().unwrap_or(0),
            y: value.1.try_into().unwrap_or(0),
        }
    }
}

impl Into<(usize, usize)> for Point {
    fn into(self) -> (usize, usize) {
        (
            self.x.try_into().unwrap_or(0),
            self.y.try_into().unwrap_or(0),
        )
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Area {
    pub origin: Point,
    pub max: Point,
}

impl Area {
    pub fn new(origin: Point, max: Point) -> Self {
        Self { origin, max }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.origin.x
            && point.y >= self.origin.y
            && point.x <= self.max.x
            && point.y <= self.max.y
    }

    pub fn height(&self) -> usize {
        (self.max.y - self.origin.y).try_into().unwrap_or(0)
    }

    pub fn width(&self) -> usize {
        (self.max.x - self.origin.x).try_into().unwrap_or(0)
    }
}

impl From<(Point, Point)> for Area {
    fn from(value: (Point, Point)) -> Self {
        Self {
            origin: value.0,
            max: value.1,
        }
    }
}

impl From<(u16, u16, u16, u16)> for Area {
    fn from(value: (u16, u16, u16, u16)) -> Self {
        Self {
            origin: Point::new(value.0, value.1),
            max: Point::new(value.2, value.3),
        }
    }
}

impl Into<(Point, Point)> for Area {
    fn into(self) -> (Point, Point) {
        (self.origin, self.max)
    }
}

impl Into<(u16, u16, u16, u16)> for Area {
    fn into(self) -> (u16, u16, u16, u16) {
        (self.origin.x, self.origin.y, self.max.x, self.max.y)
    }
}
