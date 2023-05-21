use crate::map::{Tile, Map};

pub fn make_room(style : Tile, w : i32, h : i32) -> Map {
    let shape = super::RoomShape::from(style);
    
    let mut map = Map::new(w,h);

    match shape {
        super::RoomShape::Organic => add_organic_floor(style, &mut map),
        super::RoomShape::Constructed => add_constructed_floor(style, &mut map),
    };

    add_walls(&mut map);

    map
}

fn add_organic_floor(style : Tile, map : &mut Map) {
    let x_max = map.x_max();
    let z_max = map.z_max();
    for z in 1 .. z_max - 1 {
        for x in 1 .. x_max - 1 {
            let x_border = x == 1 || x == x_max - 2;
            let z_border = z == 1 || z == z_max - 2;

            if x_border && z_border { continue }
            if (x_border || z_border) && fastrand::bool() { continue }

            map.set_tile(x, z, style);
        }
    }
}

fn add_constructed_floor(style : Tile, map : &mut Map) {
    let x_max = map.x_max();
    let z_max = map.z_max();
    for z in 1 .. z_max - 1 {
        for x in 1 .. x_max - 1 {
            map.set_tile(x, z, style);
        }
    }

    if fastrand::bool() && z_max % 2 == 1 {
        let x0 = 2;
        let x1 = x_max - 3;
        for z in (2..z_max-2).step_by(2) {
            map.set_tile(x0, z, Tile::_Void);
            map.set_tile(x1, z, Tile::_Void);
        }
    }

    if fastrand::bool() && x_max % 2 == 1 {
        let z0 = 2;
        let z1 = z_max - 3;
        for x in (2..x_max-2).step_by(2) {
            map.set_tile(x, z0, Tile::_Void);
            map.set_tile(x, z1, Tile::_Void);
        }
    }
}

fn add_walls(map : &mut Map) {
    // Add walls
    for z in 0 .. map.z_max() {
        for x in 0 .. map.x_max() {
            if !map.tile(x,z).is_solid() {
                map.set_tile_if(x - 1, z, Tile::_Wall, |o| o == Tile::_Void);
                map.set_tile_if(x + 1, z, Tile::_Wall, |o| o == Tile::_Void);
                map.set_tile_if(x, z - 1, Tile::_Wall, |o| o == Tile::_Void);
                map.set_tile_if(x, z + 1, Tile::_Wall, |o| o == Tile::_Void);
            }
        }
    }
}