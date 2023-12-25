pub mod modelgen;
pub mod spritemap;
pub mod spritemapbuilder;

use std::collections::HashMap;
use std::default::Default;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::time::{Time, Timer, TimerMode};

use self::spritemap::SpriteSeq;

#[derive(Resource, Default)]
pub struct RenderResource {
    pub sprite_cache: HashMap<Sprite3d, Handle<Mesh>>,
    pub material: Handle<StandardMaterial>,
    pub sprites: spritemap::SpriteMap,
}

impl RenderResource {
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
        let scale = key.tile.scale;

        let x0 = scale.scale(key.tile.x);
        let x1 = scale.scale(key.tile.x + 1);
        let y0 = scale.scale(key.tile.y);
        let y1 = scale.scale(key.tile.y + 1);

        let game_size = scale.game_size();
        let w2 = game_size / 2.0;
        let h2 = game_size / 2.0;
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut positions = vec![
            [-w2, -h2, 0.0],
            [w2, -h2, 0.0],
            [w2, h2, 0.0],
            [-w2, h2, 0.0],
        ];

        let mut normals = vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ];

        let mut uv0 = match key.flipped {
            true => vec![[x0, y1], [x1, y1], [x1, y0], [x0, y0]],
            false => vec![[x1, y1], [x0, y1], [x0, y0], [x1, y0]],
        };

        if key.two_sided {
            for index in 0..4 {
                positions.push(positions[index]);
                uv0.push(uv0[index]);
                normals.push([0.0, 0.0, -1.0]);
            }
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv0);

        if key.two_sided {
            mesh.set_indices(Some(Indices::U32(vec![0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7])));
        } else {
            mesh.set_indices(Some(Indices::U32(vec![0, 2, 1, 0, 3, 2])));
        }
        mesh
    }
}

#[derive(Bundle)]
pub struct Sprite3dBundle {
    pub in_level: crate::lifecycle::LevelObject,
    pub face_camera: FaceCamera,
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
    frames: SpriteSeq, // indices of all the frames in the animation
    timer: Timer,
}

impl Animation {
    pub fn new(frames: SpriteSeq, anim_speed: f32) -> Self {
        Self {
            frames,
            timer: Timer::from_seconds(anim_speed, TimerMode::Repeating),
        }
    }

    pub fn next_sprite(&mut self, sprite: Sprite3d) -> Sprite3d {
        let mut x = sprite.tile.x;

        x += 1;
        if x == self.frames.x.end {
            x = self.frames.x.start;
        };

        let tile = spritemap::SpritePos {
            x,
            y: self.frames.y,
            scale: self.frames.scale,
        };

        Sprite3d {
            flipped: false,
            two_sided: false,
            tile,
        }
    }
}

#[allow(unused)]
// Yes, I know some of these values are unused but I think they will be useful when implementing pickups.
// TODO: Check this again after pickups are implemented
#[derive(Component, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Sprite3d {
    pub tile: spritemap::SpritePos,
    pub flipped: bool,
    pub two_sided: bool,
}

impl Sprite3d {
    pub fn new(tile: spritemap::SpritePos) -> Self {
        Self {
            tile,
            flipped: false,
            two_sided: false,
        }
    }

    pub fn make_two_sided(mut self) -> Self {
        self.two_sided = true;
        self
    }

    pub fn to_sprite_bundle(
        self,
        pos: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<RenderResource>,
    ) -> Sprite3dBundle {
        Sprite3dBundle {
            in_level: crate::lifecycle::LevelObject,
            face_camera: FaceCamera,
            sprite: Sprite3d {
                tile: self.tile,
                flipped: false,
                two_sided: false,
            },
            pbr: PbrBundle {
                mesh: render_res.get_mesh(self, meshes),
                material: render_res.material.clone(),
                transform: Transform::from_translation(pos).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
        }
    }
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut Sprite3d, &mut Handle<Mesh>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<RenderResource>,
) {
    for (mut animation, mut sprite, mut mesh) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            *sprite = animation.next_sprite(*sprite);
            *mesh = render_res.get_mesh(*sprite, &mut meshes);
        }
    }
}
