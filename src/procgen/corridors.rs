use crate::map::{Map, Coords, Tile, WallTile};

use super::{style::LevelStyle, randitem::RandItem};


pub fn connect_rooms(map : &mut Map, rng : &mut fastrand::Rng, level_style : &LevelStyle, p : (Coords, Coords)) {
    let tile = *level_style.corridors.rand_front_loaded(rng);
    match super::RoomShape::from(tile) {
        super::RoomShape::Organic => connect_rooms_organic(map, rng, tile, p),
        super::RoomShape::Constructed => connect_rooms_constructed(map, rng, level_style, tile, p),
    }
}

fn connect_rooms_constructed(map: &mut Map, rng : &mut fastrand::Rng, level_style: &LevelStyle, wall: WallTile, p : (Coords, Coords)) {

    let floor_tile = Tile::Floor(super::style::wall_to_floor(wall));
    let _door_tile = if level_style.doors.is_empty() { None } else {Some(*level_style.doors.rand_front_loaded(rng)) };

    let x0 = std::cmp::min(p.0.x, p.1.x);
    let x1 = std::cmp::max(p.0.x, p.1.x);
    let z0 = std::cmp::min(p.0.z, p.1.z);
    let z1 = std::cmp::max(p.0.z, p.1.z);

    // X axis
    for x in x0 ..= x1 {
        map.set_tile_if(x, p.1.z, floor_tile, |t| t == Tile::Void);
        map.set_tile_if(x, p.1.z, floor_tile, |t| matches!(t, Tile::Wall(_)));
        // TODO: Add doors
        // TODO: Add walls
    }

    // Y axis
    for z in z0 ..= z1 {
        map.set_tile_if(p.0.x, z, floor_tile, |t| t == Tile::Void);
        map.set_tile_if(p.0.x, z, floor_tile, |t| matches!(t, Tile::Wall(_)));
        // TODO: Add doors
        // TODO: Add walls
    }
}

pub fn connect_rooms_organic(map : &mut Map, rng : &mut fastrand::Rng, wall: WallTile, p : (Coords, Coords)) {
    let floor_tile = Tile::Floor(super::style::wall_to_floor(wall));
    let (mut cur_pos,end_pos) = p;

    loop {
        let x = cur_pos.x;
        let z = cur_pos.z;

        if map.is_solid(x, z) {
            map.set_tile(x, z, floor_tile);
        }

        let delta = end_pos - cur_pos;
        if delta == Coords::ZERO { return; }

        if rng.i32(0 .. delta.x.abs() + delta.z.abs()) < delta.x.abs() {
            cur_pos.x += delta.x.signum()
        } else {
            cur_pos.z += delta.z.signum()
        }
    }
}