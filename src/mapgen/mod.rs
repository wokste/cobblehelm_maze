use crate::grid::*;
use crate::map::*;

mod corridors;
mod graph;
pub mod randitem;
mod rooms;
pub mod style;

use randitem::RandItem;

use crate::grid::GridTransform;

use self::style::LevelIndex;

pub struct MapGenResult {
    pub tilemap: Grid<Tile>,
    pub player_pos: Coords,
    pub portal_pos: tinyvec::ArrayVec<[Coords; 4]>,
    // TODO: Stuff like locations for keys and end of level positions.
}
#[derive(Copy, Clone)]
pub enum RoomShape {
    Organic,
    Constructed,
    Mirror,
    DoubleRect,
}

pub fn make_map(level: u8, level_style: LevelIndex, rng: &mut fastrand::Rng) -> MapGenResult {
    let mut map = Grid::<Tile>::new(48, 48);

    let mut graph = graph::Graph::default();

    for _ in 0..50 {
        let style = *level_style.rooms().rand_front_loaded(rng);
        let metadata = rooms::RoomMetaData::new(style, rng);
        let room = metadata.make_room(rng);

        for _ in 0..5 {
            let transform = GridTransform::make_rand(map.size(), room.size(), rng);

            if check_place_room(&mut map, &room, &transform).is_ok() {
                graph.add_node(transform.map(room.size().rand_center(rng)), metadata);
                break;
            }
        }
    }

    graph.connect_tree();
    graph.add_more_edges(rng, 0.5);

    for edge in graph.to_edges() {
        corridors::connect_rooms(&mut map, rng, edge);
    }

    let player_pos = choose_pos(&map, rng);
    let (dir_map, dist_map) = crate::grid::find_path4_to(&map, |tile| tile.is_solid(), player_pos);

    let portal_count = if level < 4 { 2 } else { 1 };
    let portal_pos = choose_portal_pos(&dist_map, rng, portal_count);

    MapGenResult {
        tilemap: map,
        player_pos,
        portal_pos,
    }
}

fn check_place_room(
    map: &mut Grid<Tile>,
    room: &Grid<Tile>,
    transform: &GridTransform,
) -> Result<(), ()> {
    for src_pos in room.size().iter() {
        let src = room[src_pos];
        let dst = transform.map(src_pos);
        let dst = map[dst];

        if src != Tile::Void && dst != Tile::Void && src != dst {
            return Result::Err(());
        }
    }

    for src_pos in room.size().iter() {
        let src = room[src_pos];
        let dst = transform.map(src_pos);
        if src != Tile::Void {
            map[dst] = src;
        }
    }

    Result::Ok(())
}

pub fn print_map(map: &Grid<Tile>) {
    for z in 0..map.z_max() {
        for x in 0..map.x_max() {
            match map[(x, z)] {
                Tile::Void => print!("[]"),
                Tile::Wall(_) => print!("<>"),
                _ => print!(".."),
            }
        }
        println!();
    }
}

fn choose_portal_pos(
    dist_map: &Grid<u32>,
    rng: &mut fastrand::Rng,
    count: u8,
) -> tinyvec::ArrayVec<[Coords; 4]> {
    let mut out = tinyvec::ArrayVec::<[Coords; 4]>::new();

    let mut positions: Vec<_> = dist_map
        .iter()
        .filter(|(_, dist)| *dist != u32::MAX)
        .collect();

    if let Some((_, max_dist)) = positions.iter().max_by_key(|(_, d)| d) {
        let required_dist = max_dist * 8 / 10;
        positions.retain(|(_, dist)| *dist >= required_dist);

        for _ in 0..count {
            let index = rng.usize(0..positions.len());
            out.push(positions[index].0);
            positions.swap_remove(index);
        }
    }
    out
}

fn choose_pos(map: &Grid<Tile>, rng: &mut fastrand::Rng) -> Coords {
    for _ in 0..1048576 {
        let pos = map.size().rand(rng);

        if map[pos].is_solid() {
            continue;
        }

        return pos;
    }
    panic!("WTF");
}
