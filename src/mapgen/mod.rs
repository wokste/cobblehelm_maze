use crate::grid::*;
use crate::map::*;
use crate::spawnobject::SpawnObject;

mod corridors;
mod graph;
mod level_transitions;
pub mod randitem;
mod rooms;
pub mod style;

use randitem::RandItem;

use crate::grid::GridTransform;

use self::style::LevelStyle;

pub struct MapGenResult {
    pub tilemap: Grid<Tile>,
    pub player_pos: Coords,
    pub spawn_objects: Vec<(Coords, SpawnObject)>,
}
#[derive(Copy, Clone)]
pub enum RoomShape {
    Organic,
    Constructed,
    Mirror,
    DoubleRect,
}

pub fn make_map(level: u8, level_style: LevelStyle, rng: &mut fastrand::Rng) -> MapGenResult {
    let mut map = Grid::<Tile>::new(48, 48);

    let mut graph = graph::Graph::default();

    let mut spawn_objects = vec![];

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
        corridors::connect_rooms(&mut map, rng, edge, &mut spawn_objects);
    }

    let player_pos = choose_pos(&map, rng);
    let (dir_map, dist_map) = crate::grid::find_path4_to(&map, |tile| tile.is_solid(), player_pos);

    level_transitions::add_level_transition_objects(&dist_map, rng, &mut spawn_objects, level);

    // TODO: only add the ice to the ice map
    add_ice(&mut map, rng);

    MapGenResult {
        tilemap: map,
        player_pos,
        spawn_objects,
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

fn add_ice(map: &mut Grid<Tile>, rng: &mut fastrand::Rng) {
    use noise::{NoiseFn, Perlin};
    const SCALE: f64 = 10.0;

    let perlin = Perlin::new(rng.u32(0..u32::MAX));

    for (pos, tile) in map.iter_mut() {
        if let Tile::Open(floor, _) = tile {
            let pos = [pos.x as f64 / SCALE, pos.z as f64 / SCALE];
            let val = perlin.get(pos);

            if val > 0.2 {
                *floor = FloorTile::Ice;
            }
        }
    }
}
