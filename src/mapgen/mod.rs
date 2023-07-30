use crate::grid::*;
use crate::map::*;

mod corridors;
mod graph;
pub mod randitem;
mod rooms;
pub mod style;

use randitem::RandItem;

use crate::grid::GridTransform;

pub struct MapGenResult {
    pub tilemap: Grid<Tile>,
    pub player_pos: Coords,
    pub stair_pos: Coords,
    // TODO: Stuff like locations for keys and end of level positions.
}
#[derive(Copy, Clone)]
pub enum RoomShape {
    Organic,
    Constructed,
    Mirror,
    DoubleRect,
}

pub fn make_map(level: u8, rng: &mut fastrand::Rng) -> MapGenResult {
    let mut map = Grid::<Tile>::new(64, 64);

    let styles = style::make_by_level(level);

    let mut graph = graph::Graph::default();

    for _ in 0..50 {
        let style = *styles.rooms.rand_front_loaded(rng);
        let room = rooms::make_room(style, rng);

        for _ in 0..5 {
            let transform = GridTransform::make_rand(map.size(), room.size(), rng);

            if check_place_room(&mut map, &room, &transform).is_ok() {
                graph.add_node(transform.map(room.size().rand_center(rng)));
                break;
            }
        }
    }

    graph.connect_tree();
    graph.add_more_edges(rng, 0.5);

    for edge in graph.to_edges() {
        corridors::connect_rooms(&mut map, rng, &styles, edge);
    }

    let (player_pos, stair_pos) = choose_start_and_end(&map, rng);

    MapGenResult {
        tilemap: map,
        player_pos,
        stair_pos,
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

fn choose_start_and_end(map: &Grid<Tile>, rng: &mut fastrand::Rng) -> (Coords, Coords) {
    let mut pair = (choose_pos(map, rng), choose_pos(map, rng));
    let mut dist = pair.0.eucledian_dist_sq(pair.1);

    for _ in 0..4 {
        let new_pair = (choose_pos(map, rng), choose_pos(map, rng));
        let new_dist = new_pair.0.eucledian_dist_sq(new_pair.1);

        if dist < new_dist {
            pair = new_pair;
            dist = new_dist;
        }
    }

    pair
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
