use std::ops::{Index, IndexMut};

use bevy::prelude::{Resource, Vec3};
use derive_more::{Add, Sub};

pub struct Map {
    tiles : Vec<Tile>,
    size : Coords,
}

impl Map {
    pub fn new(x_max : i32, z_max : i32) -> Self {
        Self {
            tiles : vec![Tile::_Void; (x_max * z_max) as usize],
            size : Coords::new(x_max, z_max),
        }
    }

    pub fn x_max(&self) -> i32 {self.size.x}
    pub fn z_max(&self) -> i32 {self.size.z}
    pub fn max(&self) -> Coords {self.size}

    fn to_index(&self, x : i32, z : i32) -> usize {
        assert!(x >= 0 && x < self.x_max() && z >= 0 && z < self.z_max());

        (x + z * self.size.x) as usize
    }

    pub fn is_solid(&self, x : i32, z : i32) -> bool {
        if x >= 0 && x < self.x_max() && z >= 0 && z < self.z_max() {
            self.tile(x,z).is_solid()
        } else {
            true
        }
    }

    pub fn tile(&self, x : i32, z : i32) -> Tile {
        self.tiles[self.to_index(x,z)]
    }

    pub fn set_tile(&mut self, x : i32, z : i32, tile : Tile) {
        let index = self.to_index(x,z);
        self.tiles[index] = tile
    }

    pub fn set_tile_if<F>(&mut self, x : i32, z : i32, tile : Tile, f : F) where F: Fn(Tile) -> bool{
        let old_tile = self.tile(x,z);

        if f(old_tile) {
            self.set_tile(x, z, tile)
        }
    }
}

impl Index<Coords> for Map {
    type Output = Tile;

    fn index(&self, c: Coords) -> &Self::Output {
        &self.tiles[self.to_index(c.x,c.z)]
    }
}

impl IndexMut<Coords> for Map {
    fn index_mut(&mut self, c: Coords) -> &mut Self::Output {
        let id: usize = self.to_index(c.x, c.z);
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

    pub fn to_vec(&self, height : f32) -> Vec3 {
        // TODO: Height
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    _Void,
    _Wall,
    Door1,
    Castle,
    TempleBrown,
    TempleGray,
    TempleGreen,
    Cave,
    Beehive,
    Flesh,
    Demonic,
    DemonicCave,
    MetalIron,
    MetalBronze,
    Chips,
    Sewer,
    SewerCave,
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        match self {
            Tile::_Wall => true,
            Tile::_Void => true,
            _ => false
        }
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::_Void }
}

#[derive(Resource)]
pub struct MapData {
    pub map : Map,
    pub player_pos : Vec3,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            map : Map::new(1, 1),
            player_pos : Vec3::ZERO,
        }
    }

}

impl MapData {
    // TODO: Reenable for the 0.2 version
    pub fn can_see_player(&self, pos : Vec3, sight_radius : f32) -> bool {
        // TODO: Better algorithm with LoS
        (pos).distance_squared(self.player_pos) < sight_radius * sight_radius
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