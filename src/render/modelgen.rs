use crate::{
    grid::Grid,
    map::{CeilingTile, FloorTile, Tile, WallTile},
};
use bevy::{
    prelude::{Mesh, Vec2, Vec3},
    render::{mesh, render_resource::PrimitiveTopology},
};

use super::spritemap::{SpriteMap, SpriteSeq, TILESET_SIZE};

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

        let scale = 64 as f32 / TILESET_SIZE as f32;

        let uv0 = Vec2::new(scale, 0.0);
        let uv1 = Vec2::new(0.0, scale);

        let id0 = self.add_vertex(p, normal, uv + uv0 + uv1);
        let id1 = self.add_vertex(p + dir0, normal, uv + uv1);
        let id2 = self.add_vertex(p + dir0 + dir1, normal, uv);
        let id3 = self.add_vertex(p + dir1, normal, uv + uv0);

        self.indices
            .extend_from_slice(&[id0, id2, id1, id0, id3, id2]);
    }
}

pub fn map_to_mesh(map: &Grid<Tile>, sprite_map: &SpriteMap, rng: &mut fastrand::Rng) -> Mesh {
    let mut builder = MeshBuilder::default();

    for pos in map.size().shrink(1).iter() {
        if let Tile::Open(floor, ceiling) = map[pos] {
            let p0 = Vec3::new(pos.x as f32, 0.0, pos.z as f32);
            // Floor tiles
            builder.add_rect(
                p0,
                Vec3::X,
                Vec3::Z,
                floor_tex_id(floor, sprite_map).to_uv(rng),
            );

            // Ceiling Tiles
            builder.add_rect(
                p0 + Vec3::Y,
                Vec3::Z,
                Vec3::X,
                ceiling_tex_id(ceiling, sprite_map).to_uv(rng),
            );

            // Wall tiles
            if let Tile::Wall(wall) = map[pos.top()] {
                builder.add_rect(
                    p0 + Vec3::X,
                    Vec3::NEG_X,
                    Vec3::Y,
                    wall_tex_id(wall, sprite_map).to_uv(rng),
                );
            }
            if let Tile::Wall(wall) = map[pos.right()] {
                builder.add_rect(
                    p0 + Vec3::X + Vec3::Z,
                    Vec3::NEG_Z,
                    Vec3::Y,
                    wall_tex_id(wall, sprite_map).to_uv(rng),
                );
            }
            if let Tile::Wall(wall) = map[pos.bottom()] {
                builder.add_rect(
                    p0 + Vec3::Z,
                    Vec3::X,
                    Vec3::Y,
                    wall_tex_id(wall, sprite_map).to_uv(rng),
                );
            }
            if let Tile::Wall(wall) = map[pos.left()] {
                builder.add_rect(
                    p0,
                    Vec3::Z,
                    Vec3::Y,
                    wall_tex_id(wall, sprite_map).to_uv(rng),
                );
            }
        }
    }

    builder.build()
}

pub fn ceiling_tex_id(tile: CeilingTile, sprite_map: &SpriteMap) -> SpriteSeq {
    let str = match tile {
        CeilingTile::White => "ceiling_white.png",
    };
    sprite_map.get_block(str)
}

pub fn floor_tex_id(tile: FloorTile, sprite_map: &SpriteMap) -> SpriteSeq {
    let str = match tile {
        FloorTile::Sand => "floor_sand.png",
        FloorTile::BrownFloor => "floor_brown.png",
        FloorTile::GrayFloor => "temple_gray_floor.png",
    };
    sprite_map.get_block(str)
}

pub fn wall_tex_id(tile: WallTile, sprite_map: &SpriteMap) -> SpriteSeq {
    let str = match tile {
        WallTile::Castle => "castle.png",
        WallTile::TempleBrown => "temple_brown.png",
        WallTile::TempleGray => "temple_gray_wall.png",
        WallTile::TempleGreen => "temple_gray_wall.png",
        WallTile::Cave => "cave_wall.png",
        WallTile::Beehive => "hive.png",
        WallTile::Demonic => "demonic_wall.png", // TODO: Asset
        WallTile::MetalIron => "plates_iron.png",
        WallTile::MetalBronze => "plates_bronze.png",
        WallTile::Sewer => "sewer_wall.png",
        WallTile::MetalCorrugated => "plates_corrugated.png",
    };
    sprite_map.get_block(str)
}
