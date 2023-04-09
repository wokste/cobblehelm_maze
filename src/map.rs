use bevy::prelude::{IVec2, Resource, Vec3};
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
    _Void,
    _Wall,
    Door1,
    Kitchen,
    Temple1,
    Temple2,
    Cave,
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        match self {
            Tile::_Wall => true,
            Tile::_Void => true,
            _ => false
        }
    }

    pub fn floor_tex_id(&self) -> IVec2 {
        match self {
            Tile::Kitchen => IVec2::new(1,1),
            Tile::Door1 => IVec2::new(1,2),
            _ => IVec2::new(0,1),
            
        }
    }

    pub fn wall_tex_id(&self) -> IVec2 {
        match self {
            Tile::Kitchen => IVec2::new(0,0),
            Tile::Cave => IVec2::new(1,0),
            Tile::Temple1 => IVec2::new(2,0),
            Tile::Temple2 => IVec2::new(3,0),
            Tile::Door1 => IVec2::new(1,2),
            _ => IVec2::new(1,2),
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
    pub fn can_see_player(&self, pos : Vec3) -> bool {
        // TODO: Better algorithm with LoS
        (pos).distance_squared(self.player_pos) < 25.0
    }
}