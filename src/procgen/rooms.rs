use std::f32::consts::TAU;

use bevy::prelude::Vec2;

use crate::map::{Tile, Map};

pub fn make_room(style : Tile) -> Map {
    let shape = super::RoomShape::from(style);
    
    let mut map = match shape {
        super::RoomShape::Organic => make_organic_floor(style, fastrand::i32(6..14), fastrand::i32(6..14)),
        super::RoomShape::Constructed => make_constructed_floor(style, fastrand::i32(4..12), fastrand::i32(4..12)),
    };

    add_walls(&mut map);

    map
}

fn make_organic_floor(style : Tile, x_max : i32, z_max : i32) -> Map {
    let mut map = Map::new(x_max + 2,z_max + 2);

    let center = Vec2::new(x_max as f32 + 1.0, z_max as f32 + 1.0) / 2.0;
    let scale = Vec2::new(2.0 / (x_max as f32), 2.0 / (z_max as f32));

    let ang_spikes = fastrand::i32(3..=5) as f32;
    let ang_offset = fastrand::f32() * TAU;

    for z in 1 .. z_max + 1 {
        for x in 1 .. x_max + 1 {
            let pos = Vec2::new(x as f32, z as f32);
            let delta = (pos - center) * scale;

            let len = delta.length();
            let angle = delta.x.atan2(delta.y);

            let len_max = (angle * ang_spikes + ang_offset).sin() * 0.25 + 0.75;

            if angle.is_nan() || len < len_max {
                map.set_tile(x, z, style);
            }
        }
    }
    map
}

fn make_constructed_floor(style : Tile, x_max : i32, z_max : i32) -> Map {
    let mut map = Map::new(x_max + 2,z_max + 2);
    for z in 1 .. z_max + 1 {
        for x in 1 .. x_max + 1 {
            map.set_tile(x, z, style);
        }
    }

    if fastrand::bool() && z_max % 2 == 1 {
        let x0 = 2;
        let x1 = x_max - 1;
        for z in (2..z_max).step_by(2) {
            map.set_tile(x0, z, Tile::_Void);
            map.set_tile(x1, z, Tile::_Void);
        }
    }

    if fastrand::bool() && x_max % 2 == 1 {
        let z0 = 2;
        let z1 = z_max - 1;
        for x in (2..x_max).step_by(2) {
            map.set_tile(x, z0, Tile::_Void);
            map.set_tile(x, z1, Tile::_Void);
        }
    }
    map
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