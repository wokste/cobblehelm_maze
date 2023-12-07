use crate::{
    combat::MonsterType,
    map::{CeilingTile, DoorType, FloorTile, WallTile},
};

use super::randitem::RandItem;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LevelIndex {
    Castle,
    Caves,
    Sewers,
    Hell,
    Machine,
}

impl LevelIndex {
    pub fn portal_sprite(self) -> &'static str {
        match self {
            Self::Castle => "portal_castle.png",
            Self::Caves => "portal_cave.png",
            Self::Sewers => "portal_sewers.png",
            Self::Hell => "portal_hell.png",
            Self::Machine => "portal_machine.png",
        }
    }

    pub fn rooms(self) -> &'static [WallTile] {
        use WallTile::*;
        match self {
            Self::Castle => &[
                Castle,
                BrownTemple,
                GrayTemple,
                GreenTemple,
                GoldBricks,
                Wood1,
                Cave,
            ],
            Self::Caves => &[
                Castle,
                Cave,
                GrayBlueTiles,
                BrownTemple,
                GrayTemple,
                Beehive,
                GreenTemple,
                GoldBricks,
                Sewer,
            ],
            Self::Sewers => &[Sewer, GreenTemple, GrayTemple, Cave, GrayBlueTiles],
            Self::Hell => &[Demonic, GrayTemple, Wood1],
            Self::Machine => &[Iron, Bronze, CorrugatedMetal, GoldBricks, GrayBlueTiles],
        }
    }

    pub fn doors(self) -> &'static [DoorType] {
        use DoorType::*;
        match self {
            Self::Castle => &[Chips],
            Self::Caves => &[],
            Self::Sewers => &[Chips],
            Self::Hell => &[],
            Self::Machine => &[Chips],
        }
    }

    pub fn monsters(self) -> &'static [MonsterType] {
        use MonsterType::*;
        match self {
            Self::Castle => &[EyeMonster1, Goblin, Imp, Laima],
            Self::Caves => &[EyeMonster1, Laima, Ettin, EyeMonster2, Goblin],
            Self::Sewers => &[Laima, EyeMonster2, Goblin, EyeMonster1],
            Self::Hell => &[Imp, EyeMonster2, Demon, Ettin],
            Self::Machine => &[IronGolem, EyeMonster2, Ettin, Demon],
        }
    }

    pub fn from_str(name: &str) -> Result<Self, String> {
        Ok(match name {
            "castle" => Self::Castle,
            "caves" => Self::Caves,
            "sewers" => Self::Sewers,
            "hell" => Self::Hell,
            "machine" => Self::Machine,
            _ => {
                return Err(format!("Level style {} unknown", name));
            }
        })
    }
}

pub fn choose_shape(tile: WallTile, rng: &mut fastrand::Rng) -> super::RoomShape {
    use super::RoomShape::*;
    let slice: &[super::RoomShape] = match tile {
        WallTile::Castle => &[Constructed, DoubleRect, Mirror],
        WallTile::BrownTemple => &[DoubleRect, Constructed],
        WallTile::GrayTemple => &[Mirror, Constructed],
        WallTile::GreenTemple => &[Constructed, DoubleRect, Mirror],
        WallTile::Demonic => &[DoubleRect, Constructed, Mirror],
        WallTile::Iron => &[Mirror, Constructed],
        WallTile::Bronze => &[Mirror, Constructed],
        WallTile::Cave => &[Organic],
        WallTile::Beehive => &[Organic],
        WallTile::Sewer => &[DoubleRect],
        WallTile::CorrugatedMetal => &[DoubleRect, Constructed],
        WallTile::GoldBricks => &[DoubleRect, Constructed],
        WallTile::GrayBlueTiles => &[DoubleRect, Constructed],
        WallTile::Wood1 => &[DoubleRect, Mirror],
    };
    *slice.rand_front_loaded(rng)
}

pub fn choose_floor(tile: WallTile, rng: &mut fastrand::Rng) -> FloorTile {
    use FloorTile::*;
    let slice: &[FloorTile] = match tile {
        WallTile::Castle => &[Sand, BrownFloor, GrayFloor],
        WallTile::BrownTemple => &[BrownFloor, Sand],
        WallTile::GrayTemple => &[GrayFloor, RainbowTiles, Sand],
        WallTile::GreenTemple => &[Sand],
        WallTile::Cave => &[Sand],
        WallTile::Beehive => &[Sand],
        WallTile::Demonic => &[Sand],
        WallTile::Iron => &[GrayFloor, RainbowTiles],
        WallTile::Bronze => &[GrayFloor, RainbowTiles],
        WallTile::Sewer => &[Sand],
        WallTile::CorrugatedMetal => &[Sand, GrayFloor],
        WallTile::GoldBricks => &[Sand],
        WallTile::GrayBlueTiles => &[GrayFloor, RainbowTiles],
        WallTile::Wood1 => &[Sand],
    };
    *slice.rand_front_loaded(rng)
}

pub fn choose_ceiling(tile: WallTile, rng: &mut fastrand::Rng) -> CeilingTile {
    let slice: &[CeilingTile] = match tile {
        _ => &[CeilingTile::White],
    };
    *slice.rand_front_loaded(rng)
}
