use crate::map::{Map, Coords, Tile};

use super::{LevelStyle, randitem::RandItem};


pub fn connect_rooms(map : &mut Map, level_style : &LevelStyle, p : (Coords, Coords)) {
    let tile = *level_style.corridors.rand_front_loaded();
    let door_tile = if level_style.doors.is_empty() { tile } else {*level_style.doors.rand_front_loaded() };

    let x0 = std::cmp::min(p.0.x, p.1.x);
    let x1 = std::cmp::max(p.0.x, p.1.x);
    let z0 = std::cmp::min(p.0.z, p.1.z);
    let z1 = std::cmp::max(p.0.z, p.1.z);

    // X axis
    for x in x0 ..= x1 {
        map.set_tile_if(x, p.1.z, tile, |t| t == Tile::_Void);
        map.set_tile_if(x, p.1.z, door_tile, |t| t == Tile::_Wall);
    }

    // Y axis
    for z in z0 ..= z1 {
        map.set_tile_if(p.0.x, z, tile, |t| t == Tile::_Void);
        map.set_tile_if(p.0.x, z, door_tile, |t| t == Tile::_Wall);
    }
}