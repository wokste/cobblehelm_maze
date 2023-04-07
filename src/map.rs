use bevy::{
    prelude::{Mesh, IVec2, Vec2, Vec3},
    render::{render_resource::PrimitiveTopology, mesh}
};
use grid::*;

pub struct Map {
    pub tiles : Grid<Tile>,

}

impl Map {
    pub fn is_solid(&self, x : usize, y : usize) -> bool {
        if x < self.tiles.cols() || y < self.tiles.rows() {
            self.tiles[x][y].is_solid()
        } else {
            true
        }
    }

}

pub struct Coords {
    pub x : usize,
    pub y : usize,
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

    fn get_tex_id(&self) -> IVec2 {
        match self {
            Tile::Floor1 => IVec2::new(0,1),
            Tile::Floor2 => IVec2::new(1,1),
            Tile::Wall1 => IVec2::new(0,0),
            Tile::Wall2 => IVec2::new(1,0),
            Tile::Wall3 => IVec2::new(2,0),
            Tile::Wall4 => IVec2::new(3,0),
            _ => IVec2::new(1,2)
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

// map_to_mesh

#[derive(Default)]
struct MeshBuilder {
    indices : Vec<u32>,
    positions : Vec<Vec3>,
    normals : Vec<Vec3>,
    uvs : Vec<Vec2>,
}

impl MeshBuilder {
    fn to_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(mesh::Indices::U32(self.indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
    
        mesh
    }

    fn add_vertex(&mut self, pos : Vec3, normal : Vec3, uv : Vec2) -> u32 {
        let id = self.positions.len() as u32;
        self.positions.push(pos);
        self.normals.push(normal);
        self.uvs.push(uv);
        id
    }

    fn add_rect(&mut self, p : Vec3, dir0 : Vec3, dir1 : Vec3, tex_id : IVec2) {
        let normal = dir0.cross(dir1);

        let uv = Vec2::new(tex_id.x as f32 / 4.0, tex_id.y as f32 / 4.0 );
        let uv0 = Vec2::new(0.25,0.0);
        let uv1 = Vec2::new(0.0,0.25);

        let id0 = self.add_vertex(p, normal, uv);
        let id1 = self.add_vertex(p + dir0, normal, uv + uv0);
        let id2 = self.add_vertex(p + dir0 + dir1, normal, uv + uv0 + uv1);
        let id3 = self.add_vertex(p + dir1, normal, uv + uv1);

        self.indices.extend_from_slice(&[id0, id1, id2, id0, id2, id3]);
    }
}

pub fn map_to_mesh(map : &Map) -> Mesh {
    let mut builder = MeshBuilder::default();

    for y in 0 .. map.tiles.rows() {
        for x in 0 .. map.tiles.cols() {
            let tile = map.tiles[x][y];
            if tile.is_solid() && !tile.is_void() {
                let p0 = Vec3::new(x as f32, 0.0, y as f32);

                if !map.is_solid(x, y - 1) {
                    builder.add_rect(p0, Vec3::Y, Vec3::X, tile.get_tex_id());
                }
                if !map.is_solid(x + 1, y) {
                    builder.add_rect(p0 + Vec3::X, Vec3::Y, Vec3::Z, tile.get_tex_id());
                }
                if !map.is_solid(x, y + 1) {
                    builder.add_rect(p0 + Vec3::X + Vec3::Z, Vec3::Y, Vec3::NEG_X, tile.get_tex_id());
                }
                if !map.is_solid(x- 1, y) {
                    builder.add_rect(p0 + Vec3::Z, Vec3::Y, Vec3::NEG_Z, tile.get_tex_id());
                }
            }
        }
    }


    builder.to_mesh()
}