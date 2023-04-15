use crate::map::*;

trait RandItem{
    type Item;

    fn rand_front_loaded(&self) -> &Self::Item;
}

impl<T> RandItem for Vec<T> {
    type Item = T;
    
    fn rand_front_loaded(&self) -> &Self::Item {
        let len = self.len();
        let id0 = fastrand::usize(0..len);
        let id1 = fastrand::usize(0..len + 1);
        &self[usize::min( id0, id1)]
    }
}

struct LevelStyle {
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

    for _ in 0 .. 100 {
        let style = *styles.rooms.rand_front_loaded();
        let room = make_room(style, fastrand::i32(6..14), fastrand::i32(6..14));

        let offset = Coords::new(
            fastrand::i32(0 .. map.x_max() - room.x_max()),
            fastrand::i32(0 .. map.z_max() - room.z_max())
        );
        
        if check_place_room(&mut map, &room, offset).is_ok() {
            centers.push(offset + room.random_square());
        }
    }

    for i in 0 .. centers.len() - 1 {
        connect_rooms(&mut map, &styles, centers[i], centers[i + 1]);
    }

    map
}

fn make_room(style : Tile, w : i32, h : i32) -> Map {
    let mut map = Map::new(w,h);

    for z in 1 .. h - 1 {
        for x in 1 .. w - 1 {
            map.set_tile(x, z, style);
        }
    }

    // Add walls
    for z in 0 .. h {
        for x in 0 .. w {
            if !map.tile(x,z).is_solid() {
                map.set_tile_if(x - 1, z, Tile::_Wall, |o| o == Tile::_Void);
                map.set_tile_if(x + 1, z, Tile::_Wall, |o| o == Tile::_Void);
                map.set_tile_if(x, z - 1, Tile::_Wall, |o| o == Tile::_Void);
                map.set_tile_if(x, z + 1, Tile::_Wall, |o| o == Tile::_Void);
            }
        }
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

fn connect_rooms(map : &mut Map, level_style : &LevelStyle, p0 : Coords, p1 : Coords) {
    let tile = *level_style.corridors.rand_front_loaded();
    let door_tile = if level_style.doors.is_empty() { tile } else {*level_style.doors.rand_front_loaded() };

    let x0 = std::cmp::min(p0.x, p1.x);
    let x1 = std::cmp::max(p0.x, p1.x);
    let z0 = std::cmp::min(p0.z, p1.z);
    let z1 = std::cmp::max(p0.z, p1.z);

    // X axis
    for x in x0 ..= x1 {
        map.set_tile_if(x, p1.z, tile, |t| t == Tile::_Void);
        map.set_tile_if(x, p1.z, door_tile, |t| t == Tile::_Wall);
    }

    // Y axis
    for z in z0 ..= z1 {
        map.set_tile_if(p0.x, z, tile, |t| t == Tile::_Void);
        map.set_tile_if(p0.x, z, door_tile, |t| t == Tile::_Wall);
    }
}