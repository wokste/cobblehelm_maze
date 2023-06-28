use bevy::prelude::{Resource, Vec3};
use crate::grid::{Grid};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    #[default]
    Void,
    Wall(WallTile),
    Floor(FloorTile),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WallTile {
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FloorTile {
    Sand,
    BlueTiles,
    BrownFloor,
    GrayFloor,
    Cave,
    Flesh,
    Demonic,
    Chips,
    Sewer,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DoorType {
    Chips,
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        match self {
            Tile::Wall(_) => true,
            Tile::Floor(_) => false,
            Tile::Void => true,
        }
    }
}

#[derive(Resource)]
pub struct MapData {
    pub map: Grid<Tile>,
    pub player_pos: Vec3,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            map: Grid::<Tile>::new(1, 1),
            player_pos: Vec3::ZERO,
        }
    }

}

impl MapData {
    pub fn can_see_player(&self, pos: Vec3, sight_radius: f32) -> bool {
        // TODO: Better algorithm with LoS
        (pos).distance_squared(self.player_pos) < sight_radius * sight_radius
    }
}