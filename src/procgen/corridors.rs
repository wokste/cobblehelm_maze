use crate::map::{Map, Coords, Tile};

use super::{LevelStyle, randitem::RandItem};


pub fn connect_rooms(map : &mut Map, level_style : &LevelStyle, p : (Coords, Coords)) {
    let tile = *level_style.corridors.rand_front_loaded();
    match super::RoomShape::from(tile) {
        super::RoomShape::Organic => connect_rooms_organic(map, tile, p),
        super::RoomShape::Constructed => connect_rooms_constructed(map, level_style, tile, p),
    }
}

fn connect_rooms_constructed(map: &mut Map, level_style: &LevelStyle, tile: Tile, p : (Coords, Coords)) {
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

pub fn connect_rooms_organic(map : &mut Map, tile: Tile, p : (Coords, Coords)) {
    let (mut cur_pos,end_pos) = p;

    loop {
        let x = cur_pos.x;
        let z = cur_pos.z;

        if map.is_solid(x, z) {
            map.set_tile(x, z, tile);
        }

        let delta = end_pos - cur_pos;
        if delta == Coords::ZERO { return; }

        if fastrand::i32(0 .. delta.x.abs() + delta.z.abs()) < delta.x.abs() {
            cur_pos.x += delta.x.signum()
        } else {
            cur_pos.z += delta.z.signum()
        }
    }
}