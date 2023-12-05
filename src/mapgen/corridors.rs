use crate::grid::{Coords, Grid};
use crate::map::{Tile, WallTile};

use super::graph::EdgeData;
use super::rooms::RoomMetaData;

pub fn connect_rooms<'a>(
    map: &mut Grid<Tile>,
    rng: &mut fastrand::Rng,
    e: EdgeData<'a, RoomMetaData>,
) {
    match e.data0.shape {
        super::RoomShape::Organic => connect_rooms_organic(map, rng, e),
        _ => connect_rooms_constructed(map, e),
    }
}

fn connect_rooms_constructed<'a>(map: &mut Grid<Tile>, e: EdgeData<'a, RoomMetaData>) {
    let tile = Tile::Open(e.data0.floor, e.data0.ceil);
    /*let _door_tile = if level_style.doors.is_empty() {
        None
    } else {
        Some(*level_style.doors.rand_front_loaded(rng))
    };*/

    let x0 = std::cmp::min(e.c0.x, e.c1.x);
    let x1 = std::cmp::max(e.c0.x, e.c1.x);
    let z0 = std::cmp::min(e.c0.z, e.c1.z);
    let z1 = std::cmp::max(e.c0.z, e.c1.z);

    let mut added_floors = vec![];

    // X axis
    for x in x0..=x1 {
        let c = Coords::new(x, e.c1.z);
        if map[c].is_solid() {
            map[c] = tile;
            added_floors.push(c);
        }
    }

    // Y axis
    for z in z0..=z1 {
        let c = Coords::new(e.c0.x, z);
        if map[c].is_solid() {
            map[c] = tile;
            added_floors.push(c);
        }
    }

    add_walls(map, added_floors, e.data0.wall, true);
}

pub fn connect_rooms_organic<'a>(
    map: &mut Grid<Tile>,
    rng: &mut fastrand::Rng,
    e: EdgeData<'a, RoomMetaData>,
) {
    let tile = Tile::Open(e.data0.floor, e.data0.ceil);
    let mut cur_pos = e.c0;
    let end_pos = e.c1;

    let mut added_floors = vec![];
    loop {
        if map[cur_pos].is_solid() {
            map[cur_pos] = tile;
            added_floors.push(cur_pos);
        }

        let delta = end_pos - cur_pos;
        if delta == Coords::ZERO {
            break;
        }

        if rng.i32(0..delta.x.abs() + delta.z.abs()) < delta.x.abs() {
            cur_pos.x += delta.x.signum()
        } else {
            cur_pos.z += delta.z.signum()
        }
    }

    // Widen corridors
    for pos in added_floors.clone() {
        let (dx, dz) = rng.choice([(-1, 0), (1, 0), (0, -1), (0, 1)]).unwrap();
        let pos = pos + Coords::new(dx, dz);

        if map[pos].is_solid() {
            map[pos] = tile;
            added_floors.push(pos);
        }
    }

    add_walls(map, added_floors, e.data0.wall, false);
}

fn add_walls(map: &mut Grid<Tile>, added_floors: Vec<Coords>, wall: WallTile, add_doors: bool) {
    for floor_pos in added_floors {
        // Add walls
        let Tile::Open(_,_) = map[floor_pos] else {continue;};

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
            if map[l].is_solid()
                && map[r].is_solid()
                && !map[t].is_solid()
                && !map[b].is_solid()
                && map[t] != map[b]
            {
                // TODO: Add door (-)
                //map[floor_pos] = Tile::Floor(FloorTile::Door);
            }

            if map[t].is_solid()
                && map[b].is_solid()
                && !map[l].is_solid()
                && !map[r].is_solid()
                && map[l] != map[r]
            {
                // TODO: Add door (|)
                //map[floor_pos] = Tile::Floor(FloorTile::Door);
            }
        }
    }
}
