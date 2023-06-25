use std::collections::HashMap;
use std::default::Default;
use std::ops::Range;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::time::{Time, Timer, TimerMode};

const TILE_X : usize = 32;
const TILE_Y : usize = 8;

#[derive(Resource, Default)]
pub struct SpriteResource {
    pub sprite_cache: HashMap<usize, Handle<Mesh>>,
    pub material: Handle<StandardMaterial>,
}

impl SpriteResource {
    pub fn get_mesh(&mut self, index : usize, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
        if let Some(handle) = self.sprite_cache.get(&index) {
            return handle.clone();
        }

        // We haven't cached the mesh yet. Generate it.
        let mesh = Self::make_mesh(index);
        let mesh = meshes.add(mesh);
        self.sprite_cache.insert(index,mesh.clone());
        mesh
    }

    fn make_mesh(index : usize) -> Mesh {
        let x = index % TILE_X;
        let y = index / TILE_X;

        let x0 = x as f32 / TILE_X as f32;
        let x1 = (x + 1) as f32 / TILE_X as f32;
        let y0 = (y) as f32 / TILE_Y as f32;
        let y1 = (y + 1) as f32 / TILE_Y as f32;

        let w2 = 1.0 / 2.0;
        let h2 = 1.0 / 2.0;
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices = vec![[-w2, -h2, 0.0], [w2, -h2, 0.0], [w2, h2, 0.0], [-w2, h2, 0.0]];

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[x0, y1], [x1, y1], [x1, y0], [x0, y0]]);
        mesh.set_indices(Some(Indices::U32( vec![0, 2, 1, 0, 3, 2] )));
        mesh
    }

}

#[derive(Clone)]
pub struct TexCoords {
    pub x : std::ops::Range<u8>,
    pub y : u8,
}

impl TexCoords {
    pub fn new(x : std::ops::Range<u8>, y : u8) -> Self {
        Self{x,y}
    }

    pub fn to_uv(&self, rng : &mut fastrand::Rng) -> Vec2 {
        let x = rng.u8(self.x.clone());
        let y = self.y;

        Vec2::new(x as f32 / TILE_X as f32, y as f32 / TILE_Y as f32)
    }

    pub fn to_sprite_bundle(
        &self,
        pos : Vec3,
        anim_speed : f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res : &mut ResMut<SpriteResource>,
    ) -> SpriteBundle {
        let frames = Range::<usize> {
            start: self.x.start as usize + self.y as usize * TILE_X,
            end: self.x.end as usize + self.y as usize * TILE_X,
        };
        let index = frames.start;

        SpriteBundle {
            in_level : crate::LevelObject,
            face_camera : FaceCamera,
            sprite: Sprite3d {
                index,
            },
            animation : Animation {
                frames,
                timer: Timer::from_seconds(anim_speed, TimerMode::Repeating),
            },
            pbr : PbrBundle {
                mesh: render_res.get_mesh(index, meshes),
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
    pub face_camera : FaceCamera,
    pub animation : Animation,
    pub sprite : Sprite3d,
    pub pbr : PbrBundle,
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
    frames: Range<usize>, // indices of all the frames in the animation
    timer: Timer,
}

#[derive(Component)]
pub struct Sprite3d {
    index : usize,
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
            sprite.index += 1;
            if sprite.index == animation.frames.end {
                sprite.index = animation.frames.start;
            }

            *mesh = render_res.get_mesh(sprite.index, &mut meshes);
        }
    }
}