use crate::{
    combat::MonsterType,
    map::{CeilingTile, DoorType, FloorTile, WallTile},
};

use super::randitem::RandItem;

pub struct LevelStyle {
    pub rooms: &'static [WallTile],
    pub doors: &'static [DoorType],
    pub monsters: &'static [MonsterType],
}

pub fn make_by_level(level: u8) -> LevelStyle {
    match level {
        1 => LevelStyle {
            // The castle
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
            rooms: &[
                WallTile::Castle,
                WallTile::Cave,
                WallTile::TempleBrown,
                WallTile::TempleGray,
                WallTile::Beehive,
                WallTile::TempleGreen,
                WallTile::Sewer,
            ],
            doors: &[],
            monsters: &[
                MonsterType::EyeMonster1,
                MonsterType::Laima,
                MonsterType::Ettin,
                MonsterType::EyeMonster2,
            ],
        },
        3 => LevelStyle {
            // The sewers
            rooms: &[
                WallTile::Sewer,
                WallTile::TempleGreen,
                WallTile::TempleGray,
                WallTile::Cave,
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
            rooms: &[WallTile::Demonic, WallTile::TempleGray],
            doors: &[DoorType::Chips],
            monsters: &[
                MonsterType::Imp,
                MonsterType::EyeMonster2,
                MonsterType::Demon,
                MonsterType::Ettin,
            ],
        },
        _ => LevelStyle {
            // Welcome to the machine
            rooms: &[
                WallTile::MetalIron,
                WallTile::MetalBronze,
                WallTile::MetalCorrugated,
                WallTile::Castle,
            ],
            doors: &[],
            monsters: &[
                MonsterType::IronGolem,
                MonsterType::EyeMonster2,
                MonsterType::Ettin,
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
        WallTile::MetalIron => &[Mirror, Constructed],
        WallTile::MetalBronze => &[Mirror, Constructed],
        WallTile::Cave => &[Organic],
        WallTile::Beehive => &[Organic],
        WallTile::Sewer => &[DoubleRect],
        WallTile::MetalCorrugated => &[DoubleRect, Constructed],
    };
    *slice.rand_front_loaded(rng)
}

pub fn choose_floor(tile: WallTile, rng: &mut fastrand::Rng) -> FloorTile {
    let slice: &[FloorTile] = match tile {
        WallTile::Castle => &[FloorTile::Sand, FloorTile::BrownFloor, FloorTile::GrayFloor],
        WallTile::TempleBrown => &[FloorTile::BrownFloor, FloorTile::Sand],
        WallTile::TempleGray => &[FloorTile::GrayFloor, FloorTile::Sand],
        WallTile::TempleGreen => &[FloorTile::Sand],
        WallTile::Cave => &[FloorTile::Sand],
        WallTile::Beehive => &[FloorTile::Sand],
        WallTile::Demonic => &[FloorTile::Sand],
        WallTile::MetalIron => &[FloorTile::GrayFloor],
        WallTile::MetalBronze => &[FloorTile::GrayFloor],
        WallTile::Sewer => &[FloorTile::Sand],
        WallTile::MetalCorrugated => &[FloorTile::Sand],
    };
    *slice.rand_front_loaded(rng)
}

pub fn choose_ceiling(tile: WallTile, rng: &mut fastrand::Rng) -> CeilingTile {
    let slice: &[CeilingTile] = match tile {
        _ => &[CeilingTile::White],
    };
    *slice.rand_front_loaded(rng)
}
