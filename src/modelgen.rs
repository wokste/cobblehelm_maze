use bevy::{
    prelude::{Mesh, Vec2, Vec3},
    render::{render_resource::PrimitiveTopology, mesh}
};
use crate::map::Map;


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

    fn add_rect(&mut self, p : Vec3, dir0 : Vec3, dir1 : Vec3, uv : Vec2) {
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

pub fn map_to_mesh(map : &Map) -> Mesh {
    let mut builder = MeshBuilder::default();

    for z in 0 .. map.z_max() {
        for x in 0 .. map.x_max() {
            let tile = map.tile(x,z);
            let p0 = Vec3::new(x as f32, 0.0, z as f32);
            if !tile.is_solid() {
                // Floor tiles
                builder.add_rect(p0, Vec3::X, Vec3::Z, tile.floor_tex_id().to_uv());

                // Wall tiles
                if map.is_solid(x, z - 1) {
                    builder.add_rect(p0 + Vec3::X, Vec3::NEG_X,Vec3::Y, tile.wall_tex_id().to_uv());
                }
                if map.is_solid(x + 1, z) {
                    builder.add_rect(p0 + Vec3::X + Vec3::Z, Vec3::NEG_Z, Vec3::Y,tile.wall_tex_id().to_uv());
                }
                if map.is_solid(x, z + 1) {
                    builder.add_rect(p0 + Vec3::Z, Vec3::X, Vec3::Y,tile.wall_tex_id().to_uv());
                }
                if map.is_solid(x- 1, z) {
                    builder.add_rect(p0,  Vec3::Z, Vec3::Y,tile.wall_tex_id().to_uv());
                }
            }
        }
    }

    builder.to_mesh()
}