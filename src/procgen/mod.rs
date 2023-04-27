use crate::map::*;

mod corridors;
mod graph;
mod randitem;
mod rooms;

use randitem::RandItem;

pub struct LevelStyle {
    corridors : Vec<Tile>,
    rooms: Vec<Tile>,
    doors : Vec<Tile>
}

pub fn make_map(level : u8) -> Map {
    let mut map = Map::new(64,64);

    let styles = match level{
        1 => LevelStyle{ // The castle
            corridors: vec![Tile::Castle],
            rooms: vec![Tile::Castle, Tile::TempleBrown, Tile::TempleGray, Tile::TempleGreen, Tile::Cave],
            doors: vec![Tile::Door1],
        },
        2 => LevelStyle{ // Caves below the castle
            corridors: vec![Tile::Cave],
            rooms: vec![Tile::Castle, Tile::Cave, Tile::TempleBrown, Tile::TempleGray, Tile::Beehive, Tile::TempleGreen],
            doors: vec![],
        },
        3 => LevelStyle{ // The sewers
            corridors: vec![Tile::Sewer],
            rooms: vec![Tile::SewerCave, Tile::TempleGreen, Tile::Sewer, Tile::TempleGray],
            doors: vec![Tile::Door1],
        },
        4 => LevelStyle{ // In hell
            corridors: vec![Tile::TempleGray],
            rooms: vec![Tile::DemonicCave, Tile::Demonic, Tile::TempleGray, Tile::Flesh],
            doors: vec![Tile::Door1],
        },
        _ => LevelStyle{ // Welcome to the machine
            corridors: vec![Tile::MetalBronze, Tile::MetalIron],
            rooms: vec![Tile::MetalIron, Tile::MetalBronze, Tile::Chips, Tile::Beehive, Tile::Castle],
            doors: vec![],
        },
    };

    let mut centers = vec![];

    for _ in 0 .. 50 {
        let style = *styles.rooms.rand_front_loaded();
        let room = rooms::make_room(style, fastrand::i32(6..14), fastrand::i32(6..14));

        for _ in 0 .. 5 {
            let offset = Coords::new(
                fastrand::i32(0 .. map.x_max() - room.x_max()),
                fastrand::i32(0 .. map.z_max() - room.z_max())
            );
            
            if check_place_room(&mut map, &room, offset).is_ok() {
                centers.push(offset + room.random_square());
                break;
            }
        }
    }

    for edge in graph::make_tree(centers) {
        corridors::connect_rooms(&mut map, &styles, edge);
    }

    map
}

fn check_place_room(map : &mut Map, room : &Map, offset : Coords) -> Result<(),()> {
    for sz in 0 .. room.z_max() {
        for sx in 0 .. room.x_max() {
            let src = room.tile(sx, sz);
            let dst = map.tile(sx + offset.x,sz + offset.z);

            if src != Tile::_Void && dst != Tile::_Void && src != dst {
                return Result::Err(());
            }
        }
    }

    for sz in 0 .. room.z_max() {
        for sx in 0 .. room.x_max() {
            let src = room.tile(sx, sz);
            if src != Tile::_Void {
                map.set_tile(sx + offset.x, sz + offset.z, src);
            }
        }
    }

    Result::Ok(())
}