use bevy::prelude::IVec2;
use grid::*;

pub struct Map {
    pub tiles : Grid<Tile>,
}

impl Map {
    pub fn is_solid(&self, x : i32, y : i32) -> bool {
        if x >= 0 && x < self.tiles.cols() as i32 && y >= 0 && y < self.tiles.rows() as i32 {
            self.tiles[x as usize][y as usize].is_solid()
        } else {
            true
        }
    }

    pub fn tile(&self, x : i32, y : i32) -> Tile {
        self.tiles[x as usize][y as usize]
    }

    pub fn set_tile(&mut self, x : i32, y : i32, tile : Tile) {
        self.tiles[x as usize][y as usize] = tile
    }
}

pub struct Coords {
    pub x : i32,
    pub y : i32,
}

impl Coords {
    pub fn new(x : i32,y : i32) -> Self {
        Self {x,y}
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