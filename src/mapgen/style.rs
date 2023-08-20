use crate::{
    combat::MonsterType,
    map::{CeilingTile, DoorType, FloorTile, WallTile},
};

use super::randitem::RandItem;

pub struct LevelStyle {
    pub corridors: &'static [WallTile],
    pub rooms: &'static [WallTile],
    pub doors: &'static [DoorType],
    pub monsters: &'static [MonsterType],
}

pub fn make_by_level(level: u8) -> LevelStyle {
    match level {
        1 => LevelStyle {
            // The castle
            corridors: &[
                WallTile::Castle,
                WallTile::TempleBrown,
                WallTile::TempleGray,
            ],
            rooms: &[
                WallTile::Castle,
                WallTile::TempleBrown,
                WallTile::TempleGray,
                WallTile::TempleGreen,
                WallTile::Cave,
            ],
            doors: &[DoorType::Chips],
            monsters: &[
                MonsterType::Imp,
                MonsterType::EyeMonster1,
                MonsterType::Laima,
            ],
        },
        2 => LevelStyle {
            // Caves below the castle
            corridors: &[WallTile::Cave, WallTile::TempleBrown, WallTile::Castle],
            rooms: &[
                WallTile::Castle,
                WallTile::Cave,
                WallTile::TempleBrown,
                WallTile::TempleGray,
                WallTile::Beehive,
                WallTile::TempleGreen,
            ],
            doors: &[],
            monsters: &[
                MonsterType::EyeMonster1,
                MonsterType::Laima,
                MonsterType::Goliath,
                MonsterType::EyeMonster2,
            ],
        },
        3 => LevelStyle {
            // The sewers
            corridors: &[WallTile::Sewer, WallTile::TempleGreen],
            rooms: &[
                WallTile::SewerCave,
                WallTile::TempleGreen,
                WallTile::Sewer,
                WallTile::TempleGray,
            ],
            doors: &[DoorType::Chips],
            monsters: &[
                MonsterType::Laima,
                MonsterType::EyeMonster2,
                MonsterType::Imp,
                MonsterType::EyeMonster1,
            ],
        },
        4 => LevelStyle {
            // In hell
            corridors: &[WallTile::TempleGray, WallTile::Demonic],
            rooms: &[
                WallTile::DemonicCave,
                WallTile::Demonic,
                WallTile::TempleGray,
                WallTile::Flesh,
            ],
            doors: &[DoorType::Chips],
            monsters: &[
                MonsterType::Imp,
                MonsterType::EyeMonster2,
                MonsterType::Demon,
                MonsterType::Goliath,
            ],
        },
        _ => LevelStyle {
            // Welcome to the machine
            corridors: &[WallTile::MetalBronze, WallTile::MetalIron],
            rooms: &[
                WallTile::MetalIron,
                WallTile::MetalBronze,
                WallTile::Chips,
                WallTile::Beehive,
                WallTile::Castle,
            ],
            doors: &[],
            monsters: &[
                MonsterType::IronGolem,
                MonsterType::EyeMonster2,
                MonsterType::Goliath,
                MonsterType::Demon,
            ],
        },
    }
}

pub fn choose_shape(tile: WallTile, rng: &mut fastrand::Rng) -> super::RoomShape {
    use super::RoomShape::*;
    let slice: &[super::RoomShape] = match tile {
        WallTile::Castle => &[Constructed, DoubleRect, Mirror],
        WallTile::TempleBrown => &[DoubleRect, Constructed],
        WallTile::TempleGray => &[Mirror, Constructed],
        WallTile::TempleGreen => &[Constructed, DoubleRect, Mirror],
        WallTile::Demonic => &[DoubleRect, Constructed, Mirror],
        WallTile::Sewer => &[Constructed],
        WallTile::MetalIron => &[Mirror, Constructed],
        WallTile::MetalBronze => &[Mirror, Constructed],
        WallTile::Cave => &[Organic],
        WallTile::Beehive => &[Organic],
        WallTile::Flesh => &[Organic],
        WallTile::DemonicCave => &[Organic],
        WallTile::Chips => &[Organic, Mirror],
        WallTile::SewerCave => &[Organic],
    };
    *slice.rand_front_loaded(rng)
}

pub fn choose_floor(tile: WallTile, rng: &mut fastrand::Rng) -> FloorTile {
    let slice: &[FloorTile] = match tile {
        WallTile::Castle => &[FloorTile::Sand, FloorTile::BrownFloor, FloorTile::GrayFloor],
        WallTile::TempleBrown => &[FloorTile::BrownFloor, FloorTile::Sand],
        WallTile::TempleGray => &[FloorTile::GrayFloor, FloorTile::Sand],
        WallTile::TempleGreen => &[FloorTile::Sand, FloorTile::Sewer],
        WallTile::Cave => &[FloorTile::Cave, FloorTile::Sand],
        WallTile::Beehive => &[FloorTile::Sand, FloorTile::Cave],
        WallTile::Flesh => &[FloorTile::Flesh],
        WallTile::Demonic => &[FloorTile::Demonic, FloorTile::Flesh],
        WallTile::DemonicCave => &[FloorTile::Demonic],
        WallTile::Chips => &[FloorTile::Chips, FloorTile::BlueTiles],
        WallTile::Sewer => &[FloorTile::Sewer],
        WallTile::SewerCave => &[FloorTile::Sewer],
        WallTile::MetalIron => &[FloorTile::BlueTiles, FloorTile::Chips],
        WallTile::MetalBronze => &[FloorTile::BlueTiles, FloorTile::Chips],
    };
    *slice.rand_front_loaded(rng)
}

pub fn choose_ceiling(tile: WallTile, rng: &mut fastrand::Rng) -> CeilingTile {
    let slice: &[CeilingTile] = match tile {
        _ => &[CeilingTile::White],
    };
    *slice.rand_front_loaded(rng)
}
