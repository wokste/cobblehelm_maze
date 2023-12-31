use std::fmt;

use bevy::prelude::Vec3;
use derive_more::{Add, Sub};

#[derive(Default, PartialEq, Eq, Add, Sub, Copy, Clone)]
pub struct Coords {
    pub x: i32,
    pub z: i32,
}

impl std::fmt::Debug for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Coords::ZERO {
            write!(f, "0")
        } else {
            write!(f, "({},{})", self.x, self.z)
        }
    }
}

impl Coords {
    pub const ZERO: Coords = Coords { x: 0, z: 0 };
    pub const INVALID: Coords = Coords { x: -1, z: -1 };

    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn from_vec(v: Vec3) -> Self {
        Self {
            x: v.x.floor() as i32,
            z: v.z.floor() as i32,
        }
    }

    pub fn to_vec(self, height: f32) -> Vec3 {
        Vec3 {
            x: self.x as f32 + 0.5,
            y: height,
            z: self.z as f32 + 0.5,
        }
    }

    pub const fn transpose(self) -> Self {
        Self {
            x: self.z,
            z: self.x,
        }
    }

    pub const fn left(self) -> Self {
        Self {
            x: self.x - 1,
            z: self.z,
        }
    }
    pub const fn right(self) -> Self {
        Self {
            x: self.x + 1,
            z: self.z,
        }
    }
    pub const fn top(self) -> Self {
        Self {
            x: self.x,
            z: self.z - 1,
        }
    }

    pub const fn bottom(self) -> Self {
        Self {
            x: self.x,
            z: self.z + 1,
        }
    }

    pub const fn split(self) -> Option<(Self, Self)> {
        if self.x == 0 || self.z == 0 {
            None
        } else {
            Some((Self::new(self.x, 0), Self::new(0, self.z)))
        }
    }

    /*
    pub fn manhattan_dist(self, other: Self) -> i32 {
        let d = self - other;
        d.x.abs() + d.z.abs()
    }
    */

    pub const fn eucledian_sq(self) -> i32 {
        self.x * self.x + self.z * self.z
    }
    pub fn eucledian_dist_sq(self, other: Self) -> i32 {
        (self - other).eucledian_sq()
    }

    pub const fn dot(self, other: Self) -> i32 {
        self.x * other.x + self.z * other.z
    }

    pub fn dot_norm(self, other: Self) -> f32 {
        let dot = self.dot(other);
        let len_sq = self.eucledian_sq() * other.eucledian_sq();
        (dot as f32) / (len_sq as f32).sqrt()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_transpose_coords() {
        let c = Coords::new(13, 37);
        assert_eq!(c.transpose().transpose(), c);
    }
}
