use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Velocity {
    pub dx: i32,
    pub dy: i32
}

impl Add for Velocity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self{ dx: self.dx + rhs.dx, dy: self.dy + rhs.dy }
    }
}

impl Sub for Velocity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self{ dx: self.dx - rhs.dx, dy: self.dy - rhs.dy }
    }
}

impl AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Self) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}

impl SubAssign for Velocity {
    fn sub_assign(&mut self, rhs: Self) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}