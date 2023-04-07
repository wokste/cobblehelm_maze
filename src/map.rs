use bevy::prelude::IVec2;
use derive_more::{Add, Sub};

pub struct Map {
    tiles : Vec<Tile>,
    size : Coords,
}

impl Map {
    pub fn new(x_max : i32, z_max : i32) -> Self {
        Self {
            tiles : vec![Tile::Void; (x_max * z_max) as usize],
            size : Coords::new(x_max, z_max),
        }
    }

    pub fn x_max(&self) -> i32 {self.size.x}
    pub fn z_max(&self) -> i32 {self.size.z}

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