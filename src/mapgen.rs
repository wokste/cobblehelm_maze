use crate::map::*;


pub fn make_map() -> Map {
    let mut map = Map::new(64,64);

    let styles = [Tile::Kitchen, Tile::Temple1, Tile::Temple2];

    let mut centers = vec![];

    for _ in 0 .. 100 {
        let style = styles[fastrand::usize(0..styles.len())];
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
        connect_rooms(&mut map, Tile::Cave, centers[i], centers[i + 1]);
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

fn connect_rooms(map : &mut Map, tile : Tile, p0 : Coords, p1 : Coords) {
    let x0 = std::cmp::min(p0.x, p1.x);
    let x1 = std::cmp::max(p0.x, p1.x);
    let z0 = std::cmp::min(p0.z, p1.z);
    let z1 = std::cmp::max(p0.z, p1.z);

    // X axis
    for x in x0 ..= x1 {
        map.set_tile_if(x, p1.z, tile, |t| t == Tile::_Void);
        map.set_tile_if(x, p1.z, Tile::Door1, |t| t == Tile::_Wall);


        //for z in corner.z - 1 ..= corner.z + 1 {
        //    map.set_tile_if(x, z, Tile::_Wall, |t| t == Tile::_Void);
        //}   
    }

    // Y axis
    for z in z0 ..= z1 {
        //map.set_tile_if(p0.x, z, tile, |t| t.is_solid());
        map.set_tile_if(p0.x, z, tile, |t| t == Tile::_Void);
        map.set_tile_if(p0.x, z, Tile::Door1, |t| t == Tile::_Wall);
        //for x in corner.x - 1 ..= corner.x + 1 {
        //    map.set_tile_if(x, z, Tile::_Wall, |t| t == Tile::_Void);
        //}   
    }
}