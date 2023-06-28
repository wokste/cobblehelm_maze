use bevy::{
    prelude::{Mesh, Vec2, Vec3},
    render::{render_resource::PrimitiveTopology, mesh}
};
use crate::{grid::{Grid, Coords}, map::{Tile, WallTile, FloorTile}, rendering::TexCoords};


#[derive(Default, Clone)]
struct MeshBuilder {
    indices: Vec<u32>,
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
}

impl MeshBuilder {
    fn build(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(mesh::Indices::U32(self.indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh
    }

    fn add_vertex(&mut self, pos: Vec3, normal: Vec3, uv: Vec2) -> u32 {
        let id = self.positions.len() as u32;
        self.positions.push(pos);
        self.normals.push(normal);
        self.uvs.push(uv);
        id
    }

    fn add_rect(&mut self, p: Vec3, dir0: Vec3, dir1: Vec3, uv: Vec2) {
        let normal = dir0.cross(dir1);

        let uv0 = Vec2::new(1.0 / 32.0,0.0);
        let uv1 = Vec2::new(0.0,1.0 / 8.0);

        let id0 = self.add_vertex(p, normal, uv + uv0 + uv1);
        let id1 = self.add_vertex(p + dir0, normal, uv + uv1);
        let id2 = self.add_vertex(p + dir0 + dir1, normal, uv);
        let id3 = self.add_vertex(p + dir1, normal, uv + uv0);

        self.indices.extend_from_slice(&[id0, id2, id1, id0, id3, id2]);
    }
}

pub fn map_to_mesh(map: &Grid<Tile>, rng: &mut fastrand::Rng) -> Mesh {
    let mut builder = MeshBuilder::default();

    for z in 1 .. map.z_max() - 1 {
        for x in 1 .. map.x_max() - 1 {
            let pos = Coords::new(x,z);

            if let Tile::Floor(floor) = map[pos] {
                
                let p0 = Vec3::new(x as f32, 0.0, z as f32);
                // Floor tiles
                builder.add_rect(p0, Vec3::X, Vec3::Z, floor_tex_id(floor).to_uv(rng));

                // Wall tiles
                if let Tile::Wall(wall) = map[pos.top()] {
                    builder.add_rect(p0 + Vec3::X, Vec3::NEG_X,Vec3::Y, wall_tex_id(wall).to_uv(rng));
                }
                if let Tile::Wall(wall) = map[pos.right()] {
                    builder.add_rect(p0 + Vec3::X + Vec3::Z, Vec3::NEG_Z, Vec3::Y, wall_tex_id(wall).to_uv(rng));
                }
                if let Tile::Wall(wall) = map[pos.bottom()] {
                    builder.add_rect(p0 + Vec3::Z, Vec3::X, Vec3::Y, wall_tex_id(wall).to_uv(rng));
                }
                if let Tile::Wall(wall) = map[pos.left()] {
                    builder.add_rect(p0,  Vec3::Z, Vec3::Y, wall_tex_id(wall).to_uv(rng));
                }
            }
        }
    }

    builder.build()
}



pub fn floor_tex_id(tile: FloorTile) -> TexCoords {
    match tile {
        FloorTile::Sand => TexCoords::new(0..8,4),
        FloorTile::BrownFloor => TexCoords::new(14..18,4),
        FloorTile::GrayFloor => TexCoords::new(22..26,4),
        FloorTile::Cave => TexCoords::new(10..14,4),
        FloorTile::Flesh => TexCoords::new(18..22,4),
        FloorTile::Demonic => TexCoords::new(26..30,4),
        FloorTile::BlueTiles => TexCoords::new(8..10,4),
        FloorTile::Chips => TexCoords::new(29..32,1),
        FloorTile::Sewer => TexCoords::new(7..11,3),
        FloorTile::Exit => TexCoords::new(31..32,2),
    }
}

pub fn wall_tex_id(tile: WallTile) -> TexCoords {
    match tile {
        WallTile::Castle => TexCoords::new(0..12,0),
        WallTile::TempleBrown => TexCoords::new(12..20,0),
        WallTile::TempleGray => TexCoords::new(20..32,0),
        WallTile::TempleGreen => TexCoords::new(0..10,2),
        WallTile::Cave => TexCoords::new(0..12,1),
        WallTile::Beehive => TexCoords::new(12..22,1),
        WallTile::Flesh => TexCoords::new(22..29,1),
        WallTile::Demonic => TexCoords::new(14..25,2),
        WallTile::DemonicCave => TexCoords::new(25..29,2),
        WallTile::MetalIron => TexCoords::new(29..30,2),
        WallTile::MetalBronze => TexCoords::new(30..31,2),
        WallTile::Chips => TexCoords::new(29..32,1),
        WallTile::Sewer => TexCoords::new(0..7,3),
        WallTile::SewerCave => TexCoords::new(7..11,3),
    }
}