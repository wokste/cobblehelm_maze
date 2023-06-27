use std::f32::consts::TAU;

use bevy::prelude::Vec2;

use crate::map::{Tile, WallTile, FloorTile};
use crate::grid::{Grid};

pub fn make_room(wall: WallTile, rng: &mut fastrand::Rng) -> Grid<Tile> {
    let shape = super::RoomShape::from(wall);
    let floor = super::style::wall_to_floor(wall);
    
    let mut map = match shape {
        super::RoomShape::Organic => make_organic_floor(floor, rng, rng.i32(6..14), rng.i32(6..14)),
        super::RoomShape::Constructed => make_constructed_floor(floor, rng, rng.i32(5..14), rng.i32(4..12)),
    };

    add_walls(&mut map, wall);

    map
}

fn make_organic_floor(floor: FloorTile, rng: &mut fastrand::Rng, x_max: i32, z_max: i32) -> Grid<Tile> {
    let mut map = Grid::<Tile>::new(x_max + 2,z_max + 2);

    let center = Vec2::new(x_max as f32 + 1.0, z_max as f32 + 1.0) / 2.0;
    let scale = Vec2::new(2.0 / (x_max as f32), 2.0 / (z_max as f32));

    let ang_spikes = rng.i32(3..=5) as f32;
    let ang_offset = rng.f32() * TAU;

    for z in 1 .. z_max + 1 {
        for x in 1 .. x_max + 1 {
            let pos = Vec2::new(x as f32, z as f32);
            let delta = (pos - center) * scale;

            let len = delta.length();
            let angle = delta.x.atan2(delta.y);

            let len_max = (angle * ang_spikes + ang_offset).sin() * 0.25 + 0.75;

            if angle.is_nan() || len < len_max {
                map[(x, z)] = Tile::Floor(floor);
            }
        }
    }
    map
}

fn make_constructed_floor(floor: FloorTile, rng: &mut fastrand::Rng, x_max: i32, z_max: i32) -> Grid<Tile> {
    let mut map = Grid::<Tile>::new(x_max + 2,z_max + 2);
    for z in 1 .. z_max + 1 {
        for x in 1 .. x_max + 1 {
            map[(x, z)] = Tile::Floor(floor);
        }
    }

    if x_max % 2 == 1 && z_max > 6 && rng.bool() {
        let z0 = 2;
        let z1 = z_max - 1;
        for x in (2..x_max).step_by(2) {
            map[(x, z0)] = Tile::Void;
            map[(x, z1)] = Tile::Void;
        }
    }
    map
}

fn add_walls(map: &mut Grid<Tile>, wall: WallTile) {
    // Add walls
    let wall = Tile::Wall(wall);
    for z in 0 .. map.z_max() {
        for x in 0 .. map.x_max() {
            let Tile::Floor(_) = map[(x,z)] else {continue;};

            for c in [(x-1,z),(x+1,z),(x,z-1),(x,z+1)] {
                if map[c] == Tile::Void {
                    map[c] = wall;
                }
            }
        }
    }
}