use bevy::{
    //input::mouse::{MouseMotion},
    prelude::*,
    //Windows,
};

use crate::physics::{PhysicsBody, MapCollisionEvent};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub keys : PlayerKeys,
    pub stats : CreatureStats,
    pub physisc : PhysicsBody,
    pub team : crate::weapon::Team,
    pub weapon : crate::weapon::Weapon,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            keys : Default::default(),
            stats : Default::default(),
            physisc : PhysicsBody::new(MapCollisionEvent::Stop),
            team : crate::weapon::Team::Players,
            weapon : crate::weapon::Weapon::new(crate::weapon::ProjectileType::BlueThing)
        }
    }
}

#[derive(Default, Bundle)]
pub struct MonsterBundle {
    pub stats : CreatureStats,
    pub sprite : Sprite,
}

#[derive(Component)]
pub struct PlayerKeys {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub rot_left: KeyCode,
    pub rot_right: KeyCode,
    pub fire: KeyCode,
}

impl Default for PlayerKeys {
    fn default() -> Self {
        Self {
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            rot_left: KeyCode::Left,
            rot_right: KeyCode::Right,
            fire: KeyCode::Space
        }
    }
}

#[derive(Component)]
pub struct CreatureStats {
    pub speed: f32,
    pub rot_rate : f32,
    pub hp: i16,
    pub hp_max: i16,
}

impl Default for CreatureStats {
    fn default() -> Self {
        Self {
            speed: 6.0,
            rot_rate : 3.5,
            hp: 20,
            hp_max: 20,
        }
    }
}

#[derive(Component, Default)]
pub struct Sprite;

pub fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&PlayerKeys, &CreatureStats, &mut Transform, &mut PhysicsBody)>,
) {
    let delta_time = time.delta_seconds();
    for (key_map, stats, mut transform, mut pb) in query.iter_mut() {
        let (_, mut rotation) = transform.rotation.to_axis_angle();

        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            if *key == key_map.forward   { velocity += forward }
            if *key == key_map.backward  { velocity -= forward }
            if *key == key_map.left      { velocity -= right }
            if *key == key_map.right     { velocity += right }
            if *key == key_map.rot_left  {
                rotation += stats.rot_rate * delta_time;
                if rotation > std::f32::consts::TAU { rotation -= std::f32::consts::TAU }
            }
            if *key == key_map.rot_right {
                rotation -= stats.rot_rate * delta_time;
                if rotation < 0.0 { rotation += std::f32::consts::TAU }
            }
        }
        transform.rotation = Quat::from_rotation_y(rotation);

        pb.velocity = velocity.normalize() * stats.speed;
    }
}