use std::ops::{Add, AddAssign, Sub, SubAssign};
use super::velocity::Velocity;
use super::Map;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn wrap(&mut self, map: &Map) -> bool {
        if self.y < 0 || self.y >= map.len() as i32 {
            //False if we are vertically outside the map.
            return false;
        }
        let size = map.first().unwrap().len() as i32;
        while self.x < 0 {
            self.x += size;
        }
        while self.x >= size as i32 {
            self.x -= size;
        }
        true
    }

    pub fn top_left() -> Position { Position{ x:0, y:0 } }
}

impl Add<Velocity> for Position {
    type Output = Self;

    fn add(self, rhs: Velocity) -> Self {
        Self{ x: self.x + rhs.dx, y: self.y + rhs.dy }
    }
}

impl Sub for Position {
    type Output = Velocity;

    fn sub(self, rhs: Self) -> Velocity {
        Velocity{ dx: self.x - rhs.x, dy: self.y - rhs.y }
    }
}

impl Sub<Velocity> for Position {
    type Output = Self;

    fn sub(self, rhs: Velocity) -> Self {
        Self{ x: self.x - rhs.dx, y: self.y - rhs.dy }
    }
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.dx;
        self.y += rhs.dy;
    }
}

impl SubAssign<Velocity> for Position {
    fn sub_assign(&mut self, rhs: Velocity) {
        self.x -= rhs.dx;
        self.y -= rhs.dy;
    }
}