use std::ops::Range;

use bevy::{prelude::*, utils::HashMap};

use super::{FaceCamera, Sprite3d, Sprite3dBundle};

pub type USprite = u8;
pub const TILESET_SIZE: u32 = 1024;
const TILESET_SIZE_F: f32 = TILESET_SIZE as f32;

#[derive(Hash, Default, PartialEq, Eq, Clone, Copy)]
pub enum SpriteScale {
    #[default]
    Basic,
    Half,
    Quarter,
}

impl SpriteScale {
    pub const fn size(&self) -> u32 {
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

    pub fn size_float(&self) -> f32 {
        self.size() as f32 / TILESET_SIZE_F
    }

    pub const fn row_capacity(&self) -> (u32, u32) {
        (TILESET_SIZE / self.size(), 64 / self.size())
    }

    pub fn scale(&self, pos: USprite) -> f32 {
        (pos as f32) * self.size_float()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct SpritePos {
    pub x: USprite,
    pub y: USprite,
    pub scale: SpriteScale,
}

impl SpritePos {
    pub fn to_uv(&self) -> Vec2 {
        assert!(self.scale == SpriteScale::Basic);
        Vec2::new(self.scale.scale(self.x), self.scale.scale(self.y))
    }
}

#[derive(Hash, PartialEq, Default, Eq, Clone)]
pub struct SpriteSeq {
    pub x: Range<USprite>,
    pub y: USprite,
    pub scale: SpriteScale,
}

impl SpriteSeq {
    fn tile(&self, x: USprite) -> SpritePos {
        SpritePos {
            x,
            y: self.y,
            scale: self.scale,
        }
    }

    pub fn tile_start(&self) -> SpritePos {
        self.tile(self.x.start)
    }

    pub fn tile_rand(&self, rng: &mut fastrand::Rng) -> SpritePos {
        self.tile(rng.u8(self.x.clone()))
    }

    pub fn to_uv(&self, rng: &mut fastrand::Rng) -> Vec2 {
        self.tile_rand(rng).to_uv()
    }

    pub fn to_sprite_bundle(
        &self,
        pos: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<super::RenderResource>,
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

#[derive(Resource, Default)]
pub struct SpriteMap {
    pub texture: Handle<Image>,
    pub blocks: HashMap<String, SpriteSeq>,
    pub items: HashMap<String, SpriteSeq>,
    pub misc: HashMap<String, SpriteSeq>,
    pub monsters: HashMap<String, SpriteSeq>,
    pub projectiles: HashMap<String, SpriteSeq>,
}
impl<'a> SpriteMap {
    pub fn find_map_mut(&'a mut self, group: SpriteGroup) -> &'a mut HashMap<String, SpriteSeq> {
        match group {
            SpriteGroup::Block => &mut self.blocks,
            SpriteGroup::Item => &mut self.items,
            SpriteGroup::Misc => &mut self.misc,
            SpriteGroup::Monster => &mut self.monsters,
            SpriteGroup::Projectile => &mut self.projectiles,
        }
    }

    pub fn get_block(&self, str: &str) -> SpriteSeq {
        if let Some(block) = self.blocks.get(str) {
            block.clone()
        } else if let Some(missing) = self.misc.get("no_block.png") {
            missing.clone()
        } else {
            panic!("Could not find block {}", str);
        }
    }

    pub fn get_item(&self, str: &str) -> SpriteSeq {
        if let Some(item) = self.items.get(str) {
            item.clone()
        } else if let Some(missing) = self.misc.get("no_item.png") {
            missing.clone()
        } else {
            panic!("Could not find item {}", str);
        }
    }

    pub fn get_monster(&self, str: &str) -> SpriteSeq {
        if let Some(monster) = self.monsters.get(str) {
            monster.clone()
        } else if let Some(missing) = self.misc.get("no_monster.png") {
            missing.clone()
        } else {
            panic!("Could not find monster {}", str);
        }
    }

    pub fn get_projectile(&self, str: &str) -> SpriteSeq {
        if let Some(projectile) = self.projectiles.get(str) {
            projectile.clone()
        } else if let Some(missing) = self.misc.get("no_projectile.png") {
            missing.clone()
        } else {
            panic!("Could not find item {}", str);
        }
    }
}

#[derive(Clone, Copy)]
pub enum SpriteGroup {
    Block,
    Item,
    Misc,
    Monster,
    Projectile,
}
