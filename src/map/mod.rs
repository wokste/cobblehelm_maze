use crate::{grid::Grid, render::spritemap::SpriteSeq};
use bevy::prelude::{Resource, Transform, Vec3};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    #[default]
    Void,
    Wall(WallTile),
    Open(FloorTile, CeilingTile),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WallTile {
    Castle,
    BrownTemple,
    GrayTemple,
    GreenTemple,
    Cave,
    Sewer,
    Beehive,
    Demonic,
    Iron,
    Bronze,
    CorrugatedMetal,
    GoldBricks,
    GrayBlueTiles,
    Wood1,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FloorTile {
    Sand,
    BrownFloor,
    GrayFloor,
    RainbowTiles,
    Ice,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CeilingTile {
    White,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DoorType {
    Wood,
}
impl DoorType {
    pub fn make_sprite(&self, sprites: &crate::render::spritemap::SpriteMap) -> SpriteSeq {
        let str = match self {
            DoorType::Wood => "door_wood1.png",
        };
        sprites.get_block(str)
    }
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        match self {
            Tile::Wall(_) => true,
            Tile::Open(_, _) => false,
            Tile::Void => true,
        }
    }

    pub fn is_on_ice(&self) -> bool {
        match self {
            Tile::Open(FloorTile::Ice, _) => true,
            _ => false,
        }
    }
}

#[derive(Resource)]
pub struct MapData {
    pub solid_map: Grid<bool>,
    pub los_map: Grid<bool>,
    pub monster_map: Grid<bool>,
    pub player_pos: Transform,
    pub tile_map: Grid<Tile>,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            solid_map: Grid::<bool>::new(1, 1),
            los_map: Grid::<bool>::new(1, 1),
            monster_map: Grid::<bool>::new(1, 1),
            player_pos: Transform::IDENTITY,
            tile_map: Grid::<Tile>::new(1, 1),
        }
    }
}

impl MapData {
    pub fn line_of_sight(&self, p0: Vec3, p1: Vec3) -> bool {
        fn make_range(f0: f32, f1: f32) -> Option<(std::ops::Range<i32>, i32)> {
            let i0 = f0.floor() as i32;
            let i1 = f1.floor() as i32;

            if i0 == i1 {
                None
            } else {
                Some(if i0 < i1 {
                    ((i0 + 1)..(i1 + 1), 0)
                } else {
                    ((i1 + 1)..(i0 + 1), -1)
                })
            }
        }

        let delta = p1 - p0;
        // Steps over X boundaries
        if let Some((range, offset)) = make_range(p0.x, p1.x) {
            // z = ax + b
            let a = delta.z / delta.x;
            let b = p0.z - a * p0.x;

            for x in range {
                let z = (a * (x as f32) + b) as i32;
                if self.los_map[(x + offset, z)] {
                    return false;
                }
            }
        }

        // Steps over Z boundaries
        if let Some((range, offset)) = make_range(p0.z, p1.z) {
            // x = az + b
            let a = delta.x / delta.z;
            let b = p0.x - a * p0.z;

            for z in range {
                let x = (a * (z as f32) + b) as i32;
                if self.los_map[(x, z + offset)] {
                    return false;
                }
            }
        }
        true
    }

    pub fn can_see_player(&self, pos: Vec3, sight_radius: f32) -> bool {
        (pos).distance_squared(self.player_pos.translation) < sight_radius * sight_radius
            && self.line_of_sight(pos, self.player_pos.translation)
    }
}
