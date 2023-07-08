use crate::grid::{Grid, Coords};
use crate::map::{Tile, WallTile};

use super::{style::LevelStyle, randitem::RandItem};


pub fn connect_rooms(map: &mut Grid<Tile>, rng: &mut fastrand::Rng, level_style: &LevelStyle, p: (Coords, Coords)) {
    let tile = *level_style.corridors.rand_front_loaded(rng);
    match super::style::choose_shape(tile, rng) {
        super::RoomShape::Organic => connect_rooms_organic(map, rng, tile, p),
        _ => connect_rooms_constructed(map, rng, level_style, tile, p),
    }
}

fn connect_rooms_constructed(map: &mut Grid<Tile>, rng: &mut fastrand::Rng, level_style: &LevelStyle, wall: WallTile, p: (Coords, Coords)) {

    let floor_tile = Tile::Floor(super::style::choose_floor(wall, rng));
    let _door_tile = if level_style.doors.is_empty() { None } else {Some(*level_style.doors.rand_front_loaded(rng)) };

    let x0 = std::cmp::min(p.0.x, p.1.x);
    let x1 = std::cmp::max(p.0.x, p.1.x);
    let z0 = std::cmp::min(p.0.z, p.1.z);
    let z1 = std::cmp::max(p.0.z, p.1.z);

    let mut added_floors = vec![];

    // X axis
    for x in x0 ..= x1 {
        let c = Coords::new(x, p.1.z);
        if map[c].is_solid() {
            map[c] = floor_tile;
            added_floors.push(c);
        }
    }

    // Y axis
    for z in z0 ..= z1 {
        let c = Coords::new(p.0.x, z);
        if map[c].is_solid() {
            map[c] = floor_tile;
            added_floors.push(c);
        }
    }

    add_walls(map, added_floors, wall, true);
}

pub fn connect_rooms_organic(map: &mut Grid<Tile>, rng: &mut fastrand::Rng, wall: WallTile, p: (Coords, Coords)) {
    let floor_tile = Tile::Floor(super::style::choose_floor(wall, rng));
    let (mut cur_pos,end_pos) = p;

    let mut added_floors = vec![];
    loop {
        if map[cur_pos].is_solid() {
            map[cur_pos] = floor_tile;
            added_floors.push(cur_pos);
        }

        let delta = end_pos - cur_pos;
        if delta == Coords::ZERO { break; }

        if rng.i32(0 .. delta.x.abs() + delta.z.abs()) < delta.x.abs() {
            cur_pos.x += delta.x.signum()
        } else {
            cur_pos.z += delta.z.signum()
        }
    }
    add_walls(map, added_floors, wall, false);
}

fn add_walls(map: &mut Grid<Tile>, added_floors: Vec<Coords>, wall: WallTile, add_doors: bool)
{
    for floor_pos in added_floors {
        // Add walls
        let Tile::Floor(_) = map[floor_pos] else {continue;};

        let l = floor_pos.left();
        let r = floor_pos.right();
        
        let t = floor_pos.top();
        let b = floor_pos.bottom();

        for wall_pos in [l, r, t, b] {
            if map[wall_pos] == Tile::Void {
                map[wall_pos] = Tile::Wall(wall);
            }
        }

        if add_doors {
            if map[l].is_solid() && map[r].is_solid() && !map[t].is_solid() && !map[b].is_solid() && map[t] != map[b] {
                // TODO: Add door (-)
                //map[floor_pos] = Tile::Floor(FloorTile::Door);
            }

            if map[t].is_solid() && map[b].is_solid() && !map[l].is_solid() && !map[r].is_solid() && map[l] != map[r] {
                // TODO: Add door (|)
                //map[floor_pos] = Tile::Floor(FloorTile::Door);
            }
        }
    }
}