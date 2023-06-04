use crate::map::*;

mod corridors;
mod graph;
mod map_transform;
mod randitem;
mod rooms;

use randitem::RandItem;

use self::map_transform::MapTransform;

pub struct LevelStyle {
    corridors : Vec<Tile>,
    rooms: Vec<Tile>,
    doors : Vec<Tile>
}

pub enum RoomShape {
    Organic,
    Constructed,
}

impl From<Tile> for RoomShape {
    fn from(tile: Tile) -> Self {
        assert!(tile != Tile::_Void);
        assert!(tile != Tile::_Wall);

        use RoomShape::*;
        match tile {
            Tile::Castle => Constructed,
            Tile::TempleBrown => Constructed,
            Tile::TempleGray => Constructed,
            Tile::TempleGreen => Constructed,
            Tile::Demonic => Constructed,
            Tile::Sewer => Constructed,
            Tile::MetalIron => Constructed,
            Tile::MetalBronze => Constructed,
            _ => Organic,
        }
    }
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
        let room = rooms::make_room(style);

        for _ in 0 .. 5 {
            let transform = MapTransform::make_rand(map.max(),room.max());
            
            if check_place_room(&mut map, &room, &transform).is_ok() {
                centers.push(transform.map(room.random_square()));
                break;
            }
        }
    }

    for edge in graph::make_tree(centers) {
        corridors::connect_rooms(&mut map, &styles, edge);
    }

    print_map(&map);

    map
}

fn check_place_room(map : &mut Map, room : &Map, transform : &MapTransform) -> Result<(),()> {
    for sz in 0 .. room.z_max() {
        for sx in 0 .. room.x_max() {
            let src = room.tile(sx, sz);
            let dst = transform.map_xz(sx,sz);
            let dst = map.tile(dst.x,dst.z);

            if src != Tile::_Void && dst != Tile::_Void && src != dst {
                return Result::Err(());
            }
        }
    }

    for sz in 0 .. room.z_max() {
        for sx in 0 .. room.x_max() {
            let src = room.tile(sx, sz);
            let dst = transform.map_xz(sx,sz);
            if src != Tile::_Void {
                map.set_tile(dst.x, dst.z, src);
            }
        }
    }

    Result::Ok(())
}

fn print_map(map : &Map) {
    for z in 0 .. map.z_max() {
        for x in 0 .. map.x_max() {
            match map.tile(x, z) {
                Tile::_Void => print!("[ ]"),
                Tile::_Wall => print!("[X]"),
                _ => print!(" . "),
            }
        }
        println!();
    }
}