use crate::map::{Tile, Map};


pub fn make_room(style : Tile, w : i32, h : i32) -> Map {
    let mut map = Map::new(w,h);

    for z in 1 .. h - 1 {
        for x in 1 .. w - 1 {
            map.set_tile(x, z, style);
        }
    }

    // Add walls
    for z in 0 .. h {
        for x in 0 .. w {
            if !map.tile(x,z).is_solid() {
                map.set_tile_if(x - 1, z, Tile::_Wall, |o| o == Tile::_Void);
                map.set_tile_if(x + 1, z, Tile::_Wall, |o| o == Tile::_Void);
                map.set_tile_if(x, z - 1, Tile::_Wall, |o| o == Tile::_Void);
                map.set_tile_if(x, z + 1, Tile::_Wall, |o| o == Tile::_Void);
            }
        }
    }

    map
}