use crate::grid::Grid;
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
    TempleBrown,
    TempleGray,
    TempleGreen,
    Cave,
    Sewer,
    Beehive,
    Demonic,
    MetalIron,
    MetalBronze,
    MetalCorrugated,
    GoldBrickWall,
    GrayBlueTiles,
    WoodWall,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FloorTile {
    Sand,
    BrownFloor,
    GrayFloor,
    RainbowTiles,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CeilingTile {
    White,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DoorType {
    Chips,
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        match self {
            Tile::Wall(_) => true,
            Tile::Open(_, _) => false,
            Tile::Void => true,
        }
    }
}

#[derive(Resource)]
pub struct MapData {
    pub solid_map: Grid<bool>,
    pub los_map: Grid<bool>,
    pub monster_map: Grid<bool>,
    pub player_pos: Transform,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            solid_map: Grid::<bool>::new(1, 1),
            los_map: Grid::<bool>::new(1, 1),
            monster_map: Grid::<bool>::new(1, 1),
            player_pos: Transform::IDENTITY,
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
