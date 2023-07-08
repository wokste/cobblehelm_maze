use std::fmt;

use super::Coords;


#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Rect {
    pub p0 : Coords,
    pub p1 : Coords,
}

impl std::fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}..{:?}]", self.p0, self.p1)
    }
}

impl Rect {
    pub fn from_xz(x: i32, z: i32) -> Self { Self { p0: Coords::ZERO, p1: Coords::new(x, z) } }

    pub fn rand_center(&self, rng: &mut fastrand::Rng) -> Coords {
        Coords::new(
            ((self.p1.x - self.p0.x) + rng.bool() as i32) / 2 + self.p0.x,
            ((self.p1.z - self.p0.z) + rng.bool() as i32) / 2 + self.p0.z
        )
    }

    pub fn rand(&self, rng: &mut fastrand::Rng) -> Coords {
        Coords::new(
            rng.i32(self.p0.x .. self.p1.x),
            rng.i32(self.p0.z .. self.p1.z),
        )
    }

    pub fn transpose(self) -> Self {
        Self {
            p0: self.p0.transpose(),
            p1: self.p1.transpose(),
        }
    }
}