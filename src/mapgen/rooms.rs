use std::f32::consts::TAU;
use std::ops::Range;

use bevy::prelude::Vec2;

use crate::map::{Tile, WallTile, FloorTile};
use crate::grid::{Grid, Rect, Coords};

pub fn make_room(wall: WallTile, rng: &mut fastrand::Rng) -> Grid<Tile> {
    let shape = super::style::choose_shape(wall, rng);
    let floor = super::style::choose_floor(wall, rng);
    
    let mut map = match shape {
        super::RoomShape::Organic => make_organic_floor(floor, rng, rng.i32(6..14), rng.i32(6..14)),
        super::RoomShape::Constructed => make_constructed_floor(floor, rng, rng.i32(5..14), rng.i32(4..12)),
        super::RoomShape::Mirror => make_mirror_floor(floor, rng, 10..20),
        super::RoomShape::DoubleRect => make_doublerect_floor(floor, rng, 5..14),
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

    for c in map.size().shrink(1).iter() {
        let pos = Vec2::new(c.x as f32, c.z as f32);
        let delta = (pos - center) * scale;

        let len = delta.length();
        let angle = delta.x.atan2(delta.y);

        let len_max = (angle * ang_spikes + ang_offset).sin() * 0.25 + 0.75;

        if angle.is_nan() || len < len_max {
            map[c] = Tile::Floor(floor);
        }
    }
    map
}

fn make_constructed_floor(floor: FloorTile, rng: &mut fastrand::Rng, mut x_max: i32, z_max: i32) -> Grid<Tile> {
    if x_max % 2 == 0 {x_max += 1;}

    let mut map = Grid::<Tile>::new(x_max + 2,z_max + 2);
    for c in map.size().shrink(1).iter() {
        map[c] = Tile::Floor(floor);
    }

    let column_pos = rng.i32(0..3);

    if column_pos > 0 && z_max > 2 + column_pos * 2 {
        let z0 = column_pos;
        let z1 = z_max + 1 - column_pos;
        for x in (2..x_max).step_by(2) {
            map[(x, z0)] = Tile::Void;
            map[(x, z1)] = Tile::Void;
        }
    }
    map
}


fn make_mirror_floor(floor: FloorTile, rng: &mut fastrand::Rng, range: Range<i32>) -> Grid<Tile> {
    let x_max = rng.i32(range) + 2;
    let z_max = rng.i32((x_max * 3/8) .. (x_max * 6/8)) + 2;

    let mut map = Grid::<Tile>::new(x_max,z_max);

    for x in 1 .. x_max - 1 {
        let dz = rng.i32(1..(z_max/2));
        for z in dz .. z_max - dz {
                map[(x, z)] = Tile::Floor(floor);
        }
    }

    // TODO: Add more quirks
    map
}


fn make_doublerect_floor(floor: FloorTile, rng: &mut fastrand::Rng, range: Range<i32>) -> Grid<Tile> {
    let x_max = rng.i32(range.clone()) + 2;
    let z_max = rng.i32(range.clone()) + 2;
    let mut map = Grid::<Tile>::new(x_max,z_max);

    let min = 4;
    let x_short = rng.i32(min..x_max);
    let z_short = rng.i32(min..z_max);

    fn align(short: i32, max: i32, rng: &mut fastrand::Rng) -> i32 {
        // TODO: Chance to align rooms with top or center
        let space = max - short;
        assert!(space > 0, "Failure in space {}={}-{}",space,max,short);
        rng.i32(0..space)
    }
    let dx = align(x_short, x_max, rng);
    let dz = align(z_short, z_max, rng);

    let rects = [
        Rect{
            p0: Coords::new(dx,0),
            p1: Coords::new(x_short + dx, z_max)
        },
        Rect{
            p0: Coords::new(0,dz),
            p1: Coords::new(x_max, z_short + dz)
        },
    ];

    for rect in rects {
        for c in rect.shrink(1).iter() {
            map[c] = Tile::Floor(floor);
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