use crate::map::*;


pub fn make_map() -> Map {
    let mut map = Map::new(64,64);

    let styles = [(Tile::Wall1,Tile::Floor2), (Tile::Wall3, Tile::Floor1), (Tile::Wall4, Tile::Floor1)];

    let mut centers = vec![];

    for _ in 0 .. 100 {
        let (wall, floor) = styles[fastrand::usize(0..styles.len())];
        let room = make_room(floor, wall, fastrand::i32(6..=10), fastrand::i32(6..=10));

        let offset = Coords::new(
            fastrand::i32(0 .. map.x_max() - room.x_max()),
            fastrand::i32(0 .. map.z_max() - room.z_max())
        );
        
        if check_place_room(&mut map, &room, offset).is_ok() {
            centers.push(offset + room.random_square());
        }
    }

    for i in 0 .. centers.len() - 1 {
        connect_rooms(&mut map, Tile::Floor1, Tile::Wall2, centers[i], centers[i + 1]);
    }

    map
}

fn make_room(floor : Tile, wall : Tile, w : i32, h : i32) -> Map {
    let mut map = Map::new(w + 2,h + 2);

    for z in 1 .. map.z_max() - 1 {
        for x in 1 .. map.x_max() - 1 {
            map.set_tile(x, z, floor);
        }
    }

    for z in 0 .. map.z_max() {
        for x in 0 .. map.x_max(){
            map.set_tile_if(x, z, wall, |o| o == Tile::Void);
        }   
    }

    map
}

fn check_place_room(map : &mut Map, room : &Map, offset : Coords) -> Result<(),()> {
    for sz in 0 .. room.z_max() {
        for sx in 0 .. room.x_max() {
            let src = room.tile(sx, sz);
            let dst = map.tile(sx + offset.x,sz + offset.z);

            if src != Tile::Void && dst != Tile::Void && src != dst {
                return Result::Err(());
            }
        }
    }

    for sz in 0 .. room.z_max() {
        for sx in 0 .. room.x_max() {
            let src = room.tile(sx, sz);
            if src != Tile::Void {
                map.set_tile(sx + offset.x, sz + offset.z, src);
            }
        }
    }

    Result::Ok(())
}

fn connect_rooms(map : &mut Map, floor : Tile, wall : Tile, p0 : Coords, p1 : Coords) {
    let x0 = std::cmp::min(p0.x, p1.x);
    let x1 = std::cmp::max(p0.x, p1.x);
    let z0 = std::cmp::min(p0.z, p1.z);
    let z1 = std::cmp::max(p0.z, p1.z);

    let corner = Coords::new(p0.x, p1.z);

    // X axis
    for x in x0 ..= x1 {
        map.set_tile_if(x, p1.z, floor, |t| t.is_solid());
        for z in corner.z - 1 ..= corner.z + 1 {
            map.set_tile_if(x, z, wall, |t| t == Tile::Void);
        }   
    }

    // Y axis
    for z in z0 ..= z1 {
        map.set_tile_if(p0.x, z,floor, |t| t.is_solid());
        for x in corner.x - 1 ..= corner.x + 1 {
            map.set_tile_if(x, z, wall, |t| t == Tile::Void);
        }   
    }
}