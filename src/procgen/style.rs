use crate::map::{WallTile, FloorTile, DoorType};


pub struct LevelStyle {
    pub corridors : Vec<WallTile>,
    pub rooms: Vec<WallTile>,
    pub doors : Vec<DoorType>,
}

pub fn make_by_level(level : u8) -> LevelStyle {
    match level{
        1 => LevelStyle{ // The castle
            corridors: vec![WallTile::Castle],
            rooms: vec![WallTile::Castle, WallTile::TempleBrown, WallTile::TempleGray, WallTile::TempleGreen, WallTile::Cave],
            doors: vec![DoorType::Chips],
        },
        2 => LevelStyle{ // Caves below the castle
            corridors: vec![WallTile::Cave],
            rooms: vec![WallTile::Castle, WallTile::Cave, WallTile::TempleBrown, WallTile::TempleGray, WallTile::Beehive, WallTile::TempleGreen],
            doors: vec![],
        },
        3 => LevelStyle{ // The sewers
            corridors: vec![WallTile::Sewer],
            rooms: vec![WallTile::SewerCave, WallTile::TempleGreen, WallTile::Sewer, WallTile::TempleGray],
            doors: vec![DoorType::Chips],
        },
        4 => LevelStyle{ // In hell
            corridors: vec![WallTile::TempleGray],
            rooms: vec![WallTile::DemonicCave, WallTile::Demonic, WallTile::TempleGray, WallTile::Flesh],
            doors: vec![DoorType::Chips],
        },
        _ => LevelStyle{ // Welcome to the machine
            corridors: vec![WallTile::MetalBronze, WallTile::MetalIron],
            rooms: vec![WallTile::MetalIron, WallTile::MetalBronze, WallTile::Chips, WallTile::Beehive, WallTile::Castle],
            doors: vec![],
        },
    }
}


pub fn wall_to_floor(tile : WallTile) -> FloorTile {
    match tile {
        WallTile::Castle => FloorTile::Sand,
        WallTile::TempleBrown => FloorTile::BrownFloor,
        WallTile::TempleGray => FloorTile::GrayFloor,
        WallTile::TempleGreen => FloorTile::Sand,
        WallTile::Cave => FloorTile::Cave,
        WallTile::Beehive => FloorTile::Sand,
        WallTile::Flesh => FloorTile::Flesh,
        WallTile::Demonic => FloorTile::Demonic,
        WallTile::DemonicCave => FloorTile::Demonic,
        WallTile::Chips => FloorTile::Chips,
        WallTile::Sewer => FloorTile::Sewer,
        WallTile::SewerCave => FloorTile::Sewer,
        WallTile::MetalIron => FloorTile::BlueTiles,
        WallTile::MetalBronze => FloorTile::BlueTiles,
    }
}
