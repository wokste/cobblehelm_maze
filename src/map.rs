use bevy::prelude::Color;
use grid::*;

pub struct Map {
    pub tiles : Grid<Tile>,

}

struct Coords {
    x : usize,
    y : usize,
}

impl Coords {
    fn new(x : usize,y : usize) -> Self {
        Self {x,y}
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Void,
    Floor1,
    Floor2,
    Wall1,
    Wall2,
    Wall3,
    Wall4,
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        match self {
            Tile::Floor1 => false,
            Tile::Floor2 => false,
            _ => true
        }
    }

    pub fn is_void(&self) -> bool { *self == Tile::Void }

    pub fn to_color(&self) -> Color {
        match self {
            Tile::Wall1 => Color::rgb(0.9, 0.9, 0.9),
            Tile::Wall2 => Color::rgb(0.8, 0.4, 0.2),
            Tile::Wall3 => Color::rgb(0.8, 0.7, 0.5),
            Tile::Wall4 => Color::rgb(0.7, 0.7, 0.6),
            _ => Color::rgb(0.3, 0.3, 0.3)
        }

    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Void }
}

// Map generation

pub fn make_map() -> Map {
    let mut tiles = Grid::new(32,32);

    add_room(&mut tiles, Tile::Floor1, Tile::Wall1, Coords::new(2, 2), Coords::new(12, 12));
    add_room(&mut tiles, Tile::Floor2, Tile::Wall1, Coords::new(17, 2), Coords::new(27, 12));
    add_room(&mut tiles, Tile::Floor2, Tile::Wall3, Coords::new(2, 17), Coords::new(12, 27));
    add_room(&mut tiles, Tile::Floor1, Tile::Wall4, Coords::new(17, 17), Coords::new(27, 27));

    // Add corridors
    add_room(&mut tiles, Tile::Floor1, Tile::Wall2, Coords::new(7, 7), Coords::new(7, 22));
    add_room(&mut tiles, Tile::Floor1, Tile::Wall2, Coords::new(7, 22), Coords::new(22, 22));
    add_room(&mut tiles, Tile::Floor1, Tile::Wall2, Coords::new(22, 7), Coords::new(22, 22));

    Map {
        tiles
    }
}

fn add_room(tiles : &mut Grid<Tile>, floor : Tile, wall : Tile, p0 : Coords, p1 : Coords) {
    for y in p0.y ..= p1.y {
        for x in p0.x ..= p1.x {
            tiles[x][y] = floor;
        }   
    }

    for y in p0.y - 1 ..= p1.y + 1 {
        for x in p0.x - 1 ..= p1.x + 1 {
            if tiles[x][y] == Tile::Void {
                tiles[x][y] = wall;
            }
        }   
    }
}