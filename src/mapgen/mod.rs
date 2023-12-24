use crate::grid::*;
use crate::map::*;
use crate::spawnobject::SpawnObject;

mod corridors;
mod graph;
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

    add_level_transition_objects(&dist_map, rng, &mut spawn_objects, level);

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

fn add_level_transition_objects(
    dist_map: &Grid<u32>,
    rng: &mut fastrand::Rng,
    spawn_objects: &mut Vec<(Coords, SpawnObject)>,
    level: u8,
) {
    let items_to_spawn = choose_level_transition_items(rng, level);
    spawn_object_instances(dist_map, rng, spawn_objects, items_to_spawn)
}

fn choose_level_transition_items(rng: &mut fastrand::Rng, level: u8) -> Vec<SpawnObject> {
    use style::{ALT_LEVELS, BASE_LEVELS};
    use SpawnObject as SO;

    let style_index = (level - 1) as usize + 1;

    if style_index < BASE_LEVELS.len() {
        let base_style = BASE_LEVELS[style_index];
        let mut portals = vec![SO::Portal { style: base_style }];

        // The last level is always the same and has no choice
        // Otherwise make a second portal to the last level
        if style_index < BASE_LEVELS.len() - 1 {
            let alt_style_index =
                (level as i32 + rng.i32(-1..=1)).clamp(0, BASE_LEVELS.len() as i32 - 1);
            let mut alt_style = BASE_LEVELS[alt_style_index as usize];

            if base_style == alt_style {
                alt_style = *ALT_LEVELS.rand_front_loaded(rng);
            }

            portals.push(SO::Portal { style: alt_style })
        }

        portals
    } else {
        // In the endboss level, add a phylactery and the lich.
        vec![SpawnObject::Phylactery]
    }
}

fn spawn_object_instances(
    dist_map: &Grid<u32>,
    rng: &mut fastrand::Rng,
    spawn_objects: &mut Vec<(Coords, SpawnObject)>,
    objects_to_spawn: Vec<SpawnObject>,
) {
    let mut positions: Vec<_> = dist_map
        .iter()
        .filter(|(_, dist)| *dist != u32::MAX)
        .collect();

    let Some((_, max_dist)) = positions.iter().max_by_key(|(_, d)| d) else {
        panic!("Can't spawn objects");
        // TODO: Maybe not panic
    };

    let required_dist = max_dist * 8 / 10;
    positions.retain(|(_, dist)| *dist >= required_dist);

    for obj in objects_to_spawn.iter() {
        let index = rng.usize(0..positions.len());
        spawn_objects.push((positions[index].0, *obj));
        positions.swap_remove(index);
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
