use std::{collections::HashMap, ops::Range};

use bevy::prelude::*;

use super::{FaceCamera, Sprite3d, Sprite3dBundle};

pub type UTile = u8;
const TILESET_SIZE: i32 = 1024;
const TILESET_SIZE_F: f32 = TILESET_SIZE as f32;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum TileScale {
    Basic,
    Half,
    Quarter,
}

impl TileScale {
    const fn size(&self) -> i32 {
        match self {
            Self::Basic => 64,
            Self::Half => 32,
            Self::Quarter => 16,
        }
    }

    pub const fn game_size(&self) -> f32 {
        match self {
            Self::Basic => 1.0,
            Self::Half => 0.5,
            Self::Quarter => 0.25,
        }
    }

    pub const fn size_float(&self) -> f32 {
        self.size() as f32 / TILESET_SIZE_F
    }

    pub fn scale(&self, pos: UTile) -> f32 {
        (pos as f32) * self.size_float()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Tile {
    pub x: UTile,
    pub y: UTile,
    pub scale: TileScale,
}

impl Tile {
    pub fn to_uv(&self, rng: &mut fastrand::Rng) -> (Vec2, f32) {
        (
            Vec2::new(self.scale.scale(self.x), self.scale.scale(self.y)),
            self.scale.size_float(),
        )
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct TileSeq {
    pub x: Range<UTile>,
    pub y: UTile,
    pub scale: TileScale,
}

impl TileSeq {
    pub fn tile_start(&self) -> Tile {
        Tile {
            x: self.x.start,
            y: self.y,
            scale: self.scale,
        }
    }

    pub fn to_uv(&self, rng: &mut fastrand::Rng) -> (Vec2, f32) {
        let x = rng.u8(self.x.clone());
        (
            Vec2::new(self.scale.scale(x), self.scale.scale(self.y)),
            self.scale.size_float(),
        )
    }

    pub fn to_sprite_bundle(
        &self,
        pos: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<super::SpriteResource>,
    ) -> Sprite3dBundle {
        let tile = self.tile_start();
        let sprite = Sprite3d {
            tile,
            flipped: false,
        };

        Sprite3dBundle {
            in_level: crate::lifecycle::LevelObject,
            face_camera: FaceCamera,
            sprite,
            pbr: PbrBundle {
                mesh: render_res.get_mesh(sprite, meshes),
                material: render_res.material.clone(),
                transform: Transform::from_translation(pos).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
        }
    }
}

pub struct TileMap {
    texture: Handle<Image>,
    tiles: HashMap<String, TileSeq>,
}

pub fn make_tilemap() -> TileMap {
    let texture = TODO;
    let mut builder =
        TextureAtlas::new_empty(texture, Vec2::new(TILESET_SIZE as f32, TILESET_SIZE as f32));
    let mut tiles = default();
    TileMap { texture, tiles }
}
