use crate::map::*;
use crate::grid::*;

mod corridors;
mod graph;
mod map_transform;
pub mod randitem;
mod rooms;
pub mod style;

use randitem::RandItem;

use self::map_transform::MapTransform;

pub struct MapGenResult {
    pub map: Grid<Tile>,
    pub player_pos: Coords,
    // TODO: Stuff like locations for keys and end of level positions.
}
pub enum RoomShape {
    Organic,
    Constructed,
}

impl From<WallTile> for RoomShape {
    fn from(tile: WallTile) -> Self {

        use RoomShape::*;
        match tile {
            WallTile::Castle => Constructed,
            WallTile::TempleBrown => Constructed,
            WallTile::TempleGray => Constructed,
            WallTile::TempleGreen => Constructed,
            WallTile::Demonic => Constructed,
            WallTile::Sewer => Constructed,
            WallTile::MetalIron => Constructed,
            WallTile::MetalBronze => Constructed,
            _ => Organic,
        }
    }
}

pub fn make_map(level: u8, rng: &mut fastrand::Rng) -> MapGenResult {
    let mut map = Grid::<Tile>::new(64,64);

    let styles = style::make_by_level(level);

    let mut graph = graph::Graph::default();

    for _ in 0 .. 50 {
        let style = *styles.rooms.rand_front_loaded(rng);
        let room = rooms::make_room(style, rng);

        for _ in 0 .. 5 {
            let transform = MapTransform::make_rand(map.max(),room.max(), rng);
            
            if check_place_room(&mut map, &room, &transform).is_ok() {
                graph.add_node(transform.map(room.max().rand_center(rng)));
                break;
            }
        }
    }

    graph.connect_tree();
    graph.add_more_edges(rng, 0.5);

    for edge in graph.to_edges() {
        corridors::connect_rooms(&mut map, rng, &styles, edge);
    }

    let (player_pos, _) = choose_start_and_end(&map, rng);

    print_map(&map);

    MapGenResult
    {
        map,
        player_pos,
    }
}

fn check_place_room(map: &mut Grid<Tile>, room: &Grid<Tile>, transform: &MapTransform) -> Result<(),()> {
    for sz in 0 .. room.z_max() {
        for sx in 0 .. room.x_max() {
            let src = room[(sx, sz)];
            let dst = transform.map_xz(sx,sz);
            let dst = map[dst];

            if src != Tile::Void && dst != Tile::Void && src != dst {
                return Result::Err(());
            }
        }
    }

    for sz in 0 .. room.z_max() {
        for sx in 0 .. room.x_max() {
            let src = room[(sx, sz)];
            let dst = transform.map_xz(sx,sz);
            if src != Tile::Void {
                map[dst] = src;
            }
        }
    }

    Result::Ok(())
}

fn print_map(map: &Grid<Tile>) {
    for z in 0 .. map.z_max() {
        for x in 0 .. map.x_max() {
            match map[(x, z)] {
                Tile::Void => print!("[]"),
                Tile::Wall(_) => print!("<>"),
                _ => print!(".."),
            }
        }
        println!();
    }
}

fn choose_start_and_end(map: &Grid<Tile>, rng: &mut fastrand::Rng) -> (Coords,Coords){
    let mut pair = (choose_pos(map, rng), choose_pos(map, rng));
    let mut dist = pair.0.eucledian_dist_sq(pair.1);

    for _ in 0 .. 4 {
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
    for _ in 0 .. 1048576 {
        let pos = map.max().rand(rng);

        if map[pos].is_solid() {
            continue;
        }

        return pos;
    }
    panic!("WTF");
}