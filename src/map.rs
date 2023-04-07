use bevy::prelude::IVec2;
use derive_more::{Add, Sub};
use grid::*;

pub struct Map {
    pub tiles : Grid<Tile>,
}

impl Map {
    pub fn z_max(&self) -> i32 {self.tiles.rows() as i32}
    pub fn x_max(&self) -> i32 {self.tiles.cols() as i32}

    pub fn is_solid(&self, x : i32, z : i32) -> bool {
        if x >= 0 && x < self.x_max() as i32 && z >= 0 && z < self.z_max() as i32 {
            self.tiles[x as usize][z as usize].is_solid()
        } else {
            true
        }
    }

    pub fn tile(&self, x : i32, z : i32) -> Tile {
        self.tiles[x as usize][z as usize]
    }

    pub fn set_tile(&mut self, x : i32, z : i32, tile : Tile) {
        self.tiles[x as usize][z as usize] = tile
    }

    pub fn set_tile_if<F>(&mut self, x : i32, z : i32, tile : Tile, f : F) where F: Fn(Tile) -> bool{
        let old_tile = self.tiles[x as usize][z as usize];

        if f(old_tile) {
            self.tiles[x as usize][z as usize] = tile
        }
    }

    pub fn random_square(&self) -> Coords {
        for _ in 0 .. 1048576 {
            let x = fastrand::i32(1 .. self.x_max() - 1);
            let z = fastrand::i32(1 .. self.z_max() - 1);

            if !self.tile(x,z).is_solid() {
                return Coords::new(x as i32, z as i32);
            }
        }
        panic!("Could not find a solid tile");
    }
}

#[derive(PartialEq, Eq, Add, Sub, Copy, Clone, Debug)]
pub struct Coords {
    pub x : i32,
    pub z : i32,
}

impl Coords {
    pub fn new(x : i32, z : i32) -> Self {
        Self {x,z}
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Void,
    Floor1,
    Floor2,
    Wall1,
    Wall2,
    Wall3,
    Wall4,
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        match self {
            Tile::Floor1 => false,
            Tile::Floor2 => false,
            _ => true
        }
    }

    pub fn is_void(&self) -> bool { *self == Tile::Void }

    pub fn get_tex_id(&self) -> IVec2 {
        match self {
            Tile::Floor1 => IVec2::new(0,1),
            Tile::Floor2 => IVec2::new(1,1),
            Tile::Wall1 => IVec2::new(0,0),
            Tile::Wall2 => IVec2::new(1,0),
            Tile::Wall3 => IVec2::new(2,0),
            Tile::Wall4 => IVec2::new(3,0),
            _ => IVec2::new(1,2)
        }
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Void }
}