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
    pub fn to_style(&self) -> &'static LevelStyle {
        &LEVEL_STYLES[*self as usize]
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

pub struct LevelStyle {
    pub portal_sprite: &'static str,
    pub rooms: &'static [WallTile],
    pub doors: &'static [DoorType],
    pub monsters: &'static [MonsterType],
}

const LEVEL_STYLES: [LevelStyle; 5] = [
    LevelStyle {
        portal_sprite: "portal_castle.png",
        // The castle
        rooms: &[
            WallTile::Castle,
            WallTile::TempleBrown,
            WallTile::TempleGray,
            WallTile::TempleGreen,
            WallTile::GoldBrickWall,
            WallTile::WoodWall,
            WallTile::Cave,
        ],
        doors: &[DoorType::Chips],
        monsters: &[
            MonsterType::EyeMonster1,
            MonsterType::Goblin,
            MonsterType::Imp,
            MonsterType::Laima,
        ],
    },
    LevelStyle {
        portal_sprite: "portal_cave.png",
        // Caves below the castle
        rooms: &[
            WallTile::Castle,
            WallTile::Cave,
            WallTile::GrayBlueTiles,
            WallTile::TempleBrown,
            WallTile::TempleGray,
            WallTile::Beehive,
            WallTile::TempleGreen,
            WallTile::GoldBrickWall,
            WallTile::Sewer,
        ],
        doors: &[],
        monsters: &[
            MonsterType::EyeMonster1,
            MonsterType::Laima,
            MonsterType::Ettin,
            MonsterType::EyeMonster2,
            MonsterType::Goblin,
        ],
    },
    LevelStyle {
        portal_sprite: "portal_sewers.png",
        // The sewers
        rooms: &[
            WallTile::Sewer,
            WallTile::TempleGreen,
            WallTile::TempleGray,
            WallTile::Cave,
            WallTile::GrayBlueTiles,
        ],
        doors: &[DoorType::Chips],
        monsters: &[
            MonsterType::Laima,
            MonsterType::EyeMonster2,
            MonsterType::Goblin,
            MonsterType::EyeMonster1,
        ],
    },
    LevelStyle {
        portal_sprite: "portal_hell.png",
        // In hell
        rooms: &[WallTile::Demonic, WallTile::TempleGray, WallTile::WoodWall],
        doors: &[DoorType::Chips],
        monsters: &[
            MonsterType::Imp,
            MonsterType::EyeMonster2,
            MonsterType::Demon,
            MonsterType::Ettin,
        ],
    },
    LevelStyle {
        portal_sprite: "portal_machine.png",
        // Welcome to the machine
        rooms: &[
            WallTile::MetalIron,
            WallTile::MetalBronze,
            WallTile::MetalCorrugated,
            WallTile::GoldBrickWall,
            WallTile::GrayBlueTiles,
        ],
        doors: &[],
        monsters: &[
            MonsterType::IronGolem,
            MonsterType::EyeMonster2,
            MonsterType::Ettin,
            MonsterType::Demon,
        ],
    },
];

pub fn choose_shape(tile: WallTile, rng: &mut fastrand::Rng) -> super::RoomShape {
    use super::RoomShape::*;
    let slice: &[super::RoomShape] = match tile {
        WallTile::Castle => &[Constructed, DoubleRect, Mirror],
        WallTile::TempleBrown => &[DoubleRect, Constructed],
        WallTile::TempleGray => &[Mirror, Constructed],
        WallTile::TempleGreen => &[Constructed, DoubleRect, Mirror],
        WallTile::Demonic => &[DoubleRect, Constructed, Mirror],
        WallTile::MetalIron => &[Mirror, Constructed],
        WallTile::MetalBronze => &[Mirror, Constructed],
        WallTile::Cave => &[Organic],
        WallTile::Beehive => &[Organic],
        WallTile::Sewer => &[DoubleRect],
        WallTile::MetalCorrugated => &[DoubleRect, Constructed],
        WallTile::GoldBrickWall => &[DoubleRect, Constructed],
        WallTile::GrayBlueTiles => &[DoubleRect, Constructed],
        WallTile::WoodWall => &[DoubleRect, Mirror],
    };
    *slice.rand_front_loaded(rng)
}

pub fn choose_floor(tile: WallTile, rng: &mut fastrand::Rng) -> FloorTile {
    let slice: &[FloorTile] = match tile {
        WallTile::Castle => &[FloorTile::Sand, FloorTile::BrownFloor, FloorTile::GrayFloor],
        WallTile::TempleBrown => &[FloorTile::BrownFloor, FloorTile::Sand],
        WallTile::TempleGray => &[
            FloorTile::GrayFloor,
            FloorTile::RainbowTiles,
            FloorTile::Sand,
        ],
        WallTile::TempleGreen => &[FloorTile::Sand],
        WallTile::Cave => &[FloorTile::Sand],
        WallTile::Beehive => &[FloorTile::Sand],
        WallTile::Demonic => &[FloorTile::Sand],
        WallTile::MetalIron => &[FloorTile::GrayFloor, FloorTile::RainbowTiles],
        WallTile::MetalBronze => &[FloorTile::GrayFloor, FloorTile::RainbowTiles],
        WallTile::Sewer => &[FloorTile::Sand],
        WallTile::MetalCorrugated => &[FloorTile::Sand, FloorTile::GrayFloor],
        WallTile::GoldBrickWall => &[FloorTile::Sand],
        WallTile::GrayBlueTiles => &[FloorTile::GrayFloor, FloorTile::RainbowTiles],
        WallTile::WoodWall => &[FloorTile::Sand],
    };
    *slice.rand_front_loaded(rng)
}

pub fn choose_ceiling(tile: WallTile, rng: &mut fastrand::Rng) -> CeilingTile {
    let slice: &[CeilingTile] = match tile {
        _ => &[CeilingTile::White],
    };
    *slice.rand_front_loaded(rng)
}
