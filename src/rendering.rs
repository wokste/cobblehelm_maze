use std::collections::HashMap;
use std::default::Default;
use std::ops::Range;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::time::{Time, Timer, TimerMode};

const TILE_X: usize = 32;
const TILE_Y: usize = 8;

#[derive(Resource, Default)]
pub struct SpriteResource {
    pub sprite_cache: HashMap<Sprite3d, Handle<Mesh>>,
    pub material: Handle<StandardMaterial>,
}

impl SpriteResource {
    pub fn get_mesh(&mut self, key: Sprite3d, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {

        if let Some(handle) = self.sprite_cache.get(&key) {
            return handle.clone();
        }

        // We haven't cached the mesh yet. Generate it.
        let mesh = Self::make_mesh(key);
        let mesh = meshes.add(mesh);
        self.sprite_cache.insert(key, mesh.clone());
        mesh
    }

    fn make_mesh(key: Sprite3d) -> Mesh {
        let (x,y,flipped,size) = match key {
            Sprite3d::Basic { x, y, flipped } => (x, y, flipped, 1.0),
            Sprite3d::Half { x, y, flipped } => (x, y, flipped, 0.5),
            Sprite3d::Quarter { x, y, flipped } => (x, y, flipped, 0.25),
        };

        let x0 = x as f32 / TILE_X as f32 * size;
        let x1 = (x + 1) as f32 / TILE_X as f32 * size;
        let y0 = (y) as f32 / TILE_Y as f32 * size;
        let y1 = (y + 1) as f32 / TILE_Y as f32 * size;

        let w2 = size / 2.0;
        let h2 = size / 2.0;
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices = vec![[-w2, -h2, 0.0], [w2, -h2, 0.0], [w2, h2, 0.0], [-w2, h2, 0.0]];

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]);

        if flipped {
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[x0, y1], [x1, y1], [x1, y0], [x0, y0]]);
        } else {
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[x1, y1], [x0, y1], [x0, y0], [x1, y0]]);
        }
        
        mesh.set_indices(Some(Indices::U32( vec![0, 2, 1, 0, 3, 2] )));
        mesh
    }

}

#[derive(Clone)]
pub struct TexCoords {
    pub x: std::ops::Range<u8>,
    pub y: u8,
}

impl TexCoords {
    pub fn new(x: std::ops::Range<u8>, y: u8) -> Self {
        Self{x,y}
    }

    pub fn to_uv(&self, rng: &mut fastrand::Rng) -> Vec2 {
        let x = rng.u8(self.x.clone());
        let y = self.y;

        Vec2::new(x as f32 / TILE_X as f32, y as f32 / TILE_Y as f32)
    }

    pub fn to_sprite_bundle(
        &self,
        pos: Vec3,
        anim_speed: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<SpriteResource>,
    ) -> SpriteBundle {
        let sprite = Sprite3d::Basic { x: self.x.start, y: self.y, flipped: false };

        SpriteBundle {
            in_level: crate::LevelObject,
            face_camera: FaceCamera,
            sprite,
            animation: Animation {
                frames: self.x.clone(),
                timer: Timer::from_seconds(anim_speed, TimerMode::Repeating),
            },
            pbr: PbrBundle {
                mesh: render_res.get_mesh(sprite, meshes),
                material: render_res.material.clone(),
                transform: Transform::from_translation(pos).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct SpriteBundle{
    pub in_level: crate::LevelObject,
    pub face_camera: FaceCamera,
    pub animation: Animation,
    pub sprite: Sprite3d,
    pub pbr: PbrBundle,
}


#[derive(Component, Default)]
pub struct FaceCamera;

pub fn face_camera(
    cam_query: Query<&Transform, With<Camera>>,
    mut query: Query<&mut Transform, (With<FaceCamera>, Without<Camera>)>,
) {
    let cam_transform = cam_query.single();
    for mut transform in query.iter_mut() {
        let mut delta = cam_transform.translation - transform.translation;
        delta.y = 0.0;
        delta += transform.translation;
        transform.look_at(delta, Vec3::Y);
    }
}

#[derive(Component)]
pub struct Animation {
    frames: Range<u8>, // indices of all the frames in the animation
    timer: Timer,
}

impl Animation {
    pub fn next_sprite(&mut self, sprite: Sprite3d) -> Sprite3d {
        let mut x = match sprite {
            Sprite3d::Basic{x, ..} => x,
            Sprite3d::Half{x, ..} => x,
            Sprite3d::Quarter{x, ..} => x,
        };

        x += 1;
        if x == self.frames.end { x = self.frames.start; };

        match sprite {
            Sprite3d::Basic {y, flipped , ..} => { Sprite3d::Basic { x, y, flipped} },
            Sprite3d::Half {y, flipped , ..} => { Sprite3d::Half { x, y, flipped} },
            Sprite3d::Quarter {y, flipped , ..} => { Sprite3d::Quarter { x, y, flipped} },
        }
    }
}

#[allow(unused)] // Yes, I know some of these values are unused but I think they will be useful when implementing pickups.
// TODO: Check this again after pickups are implemented
#[derive(Component, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Sprite3d {
    Basic{x: u8, y:u8, flipped:bool},
    Half{x: u8, y:u8, flipped:bool},
    Quarter{x: u8, y:u8, flipped:bool},
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut Sprite3d, &mut Handle<Mesh>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<SpriteResource>,
) {
    for (mut animation, mut sprite, mut mesh) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            *sprite = animation.next_sprite(*sprite);
            *mesh = render_res.get_mesh(*sprite, &mut meshes);
        }
    }
}