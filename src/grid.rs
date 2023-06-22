use std::ops::{Index, IndexMut};

use bevy::prelude::Vec3;
use derive_more::{Sub, Add};

pub struct Grid<T> {
    tiles : Vec<T>,
    size : Coords,
}

impl<T> Grid<T> where T : Default, T : Clone {
    pub fn new(x_max : i32, z_max : i32) -> Self {
        Self {
            tiles : vec![T::default(); (x_max * z_max) as usize],
            size : Coords::new(x_max, z_max),
        }
    }
}

impl<T> Grid<T> {
    pub fn x_max(&self) -> i32 {self.size.x}
    pub fn z_max(&self) -> i32 {self.size.z}
    pub fn max(&self) -> Coords {self.size}

    fn to_index(&self, x : i32, z : i32) -> usize {
        assert!(x >= 0 && x < self.x_max() && z >= 0 && z < self.z_max());

        (x + z * self.size.x) as usize
    }
}

impl<T> Index<Coords> for Grid<T> {
    type Output = T;

    fn index(&self, c: Coords) -> &Self::Output {
        &self.tiles[self.to_index(c.x,c.z)]
    }
}

impl<T> IndexMut<Coords> for Grid<T> {
    fn index_mut(&mut self, c: Coords) -> &mut Self::Output {
        let id: usize = self.to_index(c.x, c.z);
        &mut self.tiles[id]
    }
}

impl<T> Index<(i32,i32)> for Grid<T> {
    type Output = T;

    fn index(&self, c: (i32,i32)) -> &Self::Output {
        &self.tiles[self.to_index(c.0,c.1)]
    }
}

impl<T> IndexMut<(i32,i32)> for Grid<T> {
    fn index_mut(&mut self, c: (i32,i32)) -> &mut Self::Output {
        let id: usize = self.to_index(c.0, c.1);
        &mut self.tiles[id]
    }
}

#[derive(PartialEq, Eq, Add, Sub, Copy, Clone, Debug)]
pub struct Coords {
    pub x : i32,
    pub z : i32,
}

impl Coords {
    pub const ZERO : Coords = Coords{x:0,z:0};

    pub fn new(x : i32, z : i32) -> Self {
        Self {x,z}
    }

    pub fn from_vec(v : Vec3) -> Self {
        Self {x : v.x.floor() as i32, z : v.z.floor() as i32}
    }

    pub fn to_vec(self, height : f32) -> Vec3 {
        Vec3 {
            x : self.x as f32 + 0.5,
            y: height,
            z : self.z as f32 + 0.5
        }
    }

    pub fn rand_center(&self, rng : &mut fastrand::Rng) -> Coords {
        Coords::new(
            (self.x + rng.bool() as i32) / 2,
            (self.z + rng.bool() as i32) / 2
        )
    }

    pub fn rand(&self, rng : &mut fastrand::Rng) -> Coords {
        Coords::new(
            rng.i32(0..self.x),
            rng.i32(0..self.z),
        )
    }

    pub fn transpose(self) -> Self { Self {x : self.z, z : self.x } }

    
    pub fn left(self) -> Self { Self {x : self.x - 1, z : self.z } }
    pub fn right(self) -> Self { Self {x : self.x + 1, z : self.z } }
    pub fn top(self) -> Self { Self {x : self.x, z : self.z - 1} }
    pub fn bottom(self) -> Self { Self {x : self.x, z : self.z + 1} }

    /*
    pub fn manhattan_dist(self, other : Self) -> i32 {
        let d = self - other;
        d.x.abs() + d.z.abs()
    }
    */

    pub fn eucledian_dist_sq(self, other : Self) -> i32 {
        let d = self - other;
        d.x * d.x + d.z * d.z
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_transpose_coords() {
        let c = Coords::new(13,37);
        assert_eq!(c.transpose().transpose(), c);
    }
} 