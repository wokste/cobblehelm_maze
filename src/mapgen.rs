use grid::Grid;
use crate::map::*;

pub fn make_map() -> Map {
    let mut map = Map{
        tiles : Grid::new(32,32),
    };

    add_room(&mut map, Tile::Floor1, Tile::Wall1, Coords::new(2, 2), Coords::new(12, 12));

    add_room(&mut map, Tile::Floor2, Tile::Wall1, Coords::new(18, 1), Coords::new(26, 13));
    add_room(&mut map, Tile::Floor2, Tile::Wall1, Coords::new(16, 3), Coords::new(28, 11));

    add_room(&mut map, Tile::Floor2, Tile::Wall3, Coords::new(2, 20), Coords::new(7, 27));
    add_room(&mut map, Tile::Floor2, Tile::Wall3, Coords::new(5, 17), Coords::new(9, 27));

    add_room(&mut map, Tile::Floor1, Tile::Wall4, Coords::new(17, 17), Coords::new(27, 27));

    add_room(&mut map, Tile::Floor1, Tile::Wall2, Coords::new(12, 20), Coords::new(14, 24));

    // Add corridors
    add_room(&mut map, Tile::Floor1, Tile::Wall2, Coords::new(7, 7), Coords::new(7, 22));
    add_room(&mut map, Tile::Floor1, Tile::Wall2, Coords::new(7, 22), Coords::new(22, 22));
    add_room(&mut map, Tile::Floor1, Tile::Wall2, Coords::new(20, 7), Coords::new(20, 20));
    add_room(&mut map, Tile::Floor1, Tile::Wall1, Coords::new(7, 7), Coords::new(22, 7));


    map
}

fn add_room(map : &mut Map, floor : Tile, wall : Tile, p0 : Coords, p1 : Coords) {
    for y in p0.y ..= p1.y {
        for x in p0.x ..= p1.x {
            if map.tile(x,y).is_solid() {
                map.set_tile(x, y,floor);
            }
        }   
    }

    for y in p0.y - 1 ..= p1.y + 1 {
        for x in p0.x - 1 ..= p1.x + 1 {
            if map.tile(x,y) == Tile::Void {
                map.set_tile(x, y, wall);
            }
        }   
    }
}