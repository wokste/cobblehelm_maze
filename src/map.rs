use bevy::prelude::{Resource, Vec3, Vec2};
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

    pub fn from_vec(v : Vec3) -> Self {
        Self {x : v.x.floor() as i32, z : v.z.floor() as i32}
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

    pub fn floor_tex_id(&self) -> TexCoords {
        match self {
            Tile::Door1 => TexCoords::new(29..32,1),
            Tile::Castle => TexCoords::new(0..8,4),
            Tile::TempleBrown => TexCoords::new(14..18,4),
            Tile::TempleGray => TexCoords::new(22..26,4),
            Tile::TempleGreen => TexCoords::new(0..8,4),
            Tile::Cave => TexCoords::new(10..14,4),
            Tile::Beehive => TexCoords::new(0..8,4),
            Tile::Flesh => TexCoords::new(18..22,4),
            Tile::Demonic => TexCoords::new(26..30,4),
            Tile::DemonicCave => TexCoords::new(26..30,4),
            Tile::MetalIron => TexCoords::new(8..10,4),
            Tile::MetalBronze => TexCoords::new(8..10,4),
            Tile::Sewer => TexCoords::new(7..11,3),
            Tile::SewerCave => TexCoords::new(7..11,3),
            _ => TexCoords::new(0..8,4),
            
        }
    }

    pub fn wall_tex_id(&self) -> TexCoords {
        match self {
            Tile::Door1 => TexCoords::new(29..32,1), // TODO: Better door tile
            Tile::Castle => TexCoords::new(0..12,0),
            Tile::TempleBrown => TexCoords::new(12..20,0),
            Tile::TempleGray => TexCoords::new(20..32,0),
            Tile::TempleGreen => TexCoords::new(0..10,2),
            Tile::Cave => TexCoords::new(0..12,1),
            Tile::Beehive => TexCoords::new(12..22,1),
            Tile::Flesh => TexCoords::new(22..29,1),
            Tile::Demonic => TexCoords::new(14..25,2),
            Tile::DemonicCave => TexCoords::new(25..29,2),
            Tile::MetalIron => TexCoords::new(29..30,2),
            Tile::MetalBronze => TexCoords::new(30..31,2),
            Tile::Sewer => TexCoords::new(0..7,3),
            Tile::SewerCave => TexCoords::new(7..11,3),
            _ => TexCoords::new(0..8,4),
        }
    }
}

#[derive(Clone)]
pub struct TexCoords {
    pub x : std::ops::Range<u8>,
    pub y : u8,
}

impl TexCoords {
    fn new(x : std::ops::Range<u8>, y : u8) -> Self {
        Self{x,y}
    }

    pub fn to_uv(&self) -> Vec2 {
        let x = fastrand::u8(self.x.clone());
        let y = self.y;

        Vec2::new(x as f32 / 32.0, y as f32 / 8.0)
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