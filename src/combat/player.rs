use bevy::{
    prelude::*,
};

use crate::{
    physics::{PhysicsBody, MapCollisionEvent, PhysicsMovable},
};

use super::{
    CreatureStats,
    weapon::{Weapon, ProjectileType, FireMode},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub keys: PlayerKeys,
    pub stats: CreatureStats,
    pub physisc: PhysicsBody,
    pub velocity: PhysicsMovable,
    pub weapon: Weapon,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            keys: Default::default(),
            stats: CreatureStats::player(),
            physisc: PhysicsBody::new(0.125, MapCollisionEvent::Stop),
            weapon: Weapon::new(ProjectileType::BlueBlob, 0.3),
            velocity: PhysicsMovable::default(),
        }
    }
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

    pub rot_rate: f32,
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
            fire: KeyCode::Space,
            
            rot_rate: 3.5,
        }
    }
}

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&PlayerKeys, &CreatureStats, &mut Transform, &mut PhysicsMovable, &mut Weapon)>,
) {
    let delta_time = time.delta_seconds();
    for (key_map, stats, mut transform, mut movable, mut weapon) in query.iter_mut() {
        let (_, mut rotation) = transform.rotation.to_axis_angle();

        let mut firing = FireMode::NoFire ;
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
                rotation += key_map.rot_rate * delta_time;
                if rotation > std::f32::consts::TAU { rotation -= std::f32::consts::TAU }
            }
            if *key == key_map.rot_right {
                rotation -= key_map.rot_rate * delta_time;
                if rotation < 0.0 { rotation += std::f32::consts::TAU }
            }
            if *key == key_map.fire      { firing = FireMode::Fire }
        }
        transform.rotation = Quat::from_rotation_y(rotation);

        movable.velocity = velocity.normalize() * stats.speed;
        weapon.set_fire_state(firing);
    }
}

pub fn update_map(
    mut map_data: ResMut<crate::map::MapData>,
    player_query: Query<&Transform, With<PlayerKeys>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<PlayerKeys>)>,
) {
    let player_transform = player_query.get_single().unwrap();
    map_data.player_pos = player_transform.translation;
    let mut camera_transform = camera_query.get_single_mut().unwrap();
    *camera_transform = *player_transform;
}