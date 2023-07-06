use bevy::{
    prelude::*, input::mouse::MouseMotion, ecs::event::ManualEventReader,
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
    pub player: Player,
    pub stats: CreatureStats,
    pub physisc: PhysicsBody,
    pub velocity: PhysicsMovable,
    pub weapon: Weapon,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player{},
            stats: CreatureStats::player(),
            physisc: PhysicsBody::new(0.125, MapCollisionEvent::Stop),
            weapon: Weapon::new(ProjectileType::BlueBlob, 0.3, 12.0),
            velocity: PhysicsMovable::default(),
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct InputMap {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub rot_left: KeyCode,
    pub rot_right: KeyCode,
    pub fire: KeyCode,

    pub rot_rate_key: f32,
    pub rot_rate_mouse: f32,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            forward: KeyCode::W,
            backward: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            rot_left: KeyCode::Left,
            rot_right: KeyCode::Right,
            fire: KeyCode::Space,
            
            rot_rate_key: 2.5,
            rot_rate_mouse: 0.1,
        }
    }
}

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource,Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mouse_motion: Res<Events<MouseMotion>>,
    mut state: ResMut<InputState>,
    time: Res<Time>,
    mut query: Query<(&CreatureStats, &mut Transform, &mut PhysicsMovable, &mut Weapon), With<Player>>,
    key_map: Res<InputMap>,
) {
    let delta_time = time.delta_seconds();
    let mut state_delta = state.as_mut();
    for (stats, mut transform, mut movable, mut weapon) in query.iter_mut() {
        let mut firing = FireMode::NoFire ;
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for ev in state_delta.reader_motion.iter(&mouse_motion) {
            state_delta.pitch -= (key_map.rot_rate_mouse * ev.delta.y).to_radians();
            state_delta.yaw -= (key_map.rot_rate_mouse * ev.delta.x).to_radians();
        }

        for key in keys.get_pressed() {
            if *key == key_map.forward   { velocity += forward }
            if *key == key_map.backward  { velocity -= forward }
            if *key == key_map.left      { velocity -= right }
            if *key == key_map.right     { velocity += right }
            if *key == key_map.rot_left  {
                state_delta.yaw += key_map.rot_rate_key * delta_time;
            }
            if *key == key_map.rot_right {
                state_delta.yaw -= key_map.rot_rate_key * delta_time;
            }
            if *key == key_map.fire      { firing = FireMode::Fire }
        }

        if state_delta.yaw > std::f32::consts::TAU { state_delta.yaw -= std::f32::consts::TAU }
        if state_delta.yaw < 0.0 { state_delta.yaw += std::f32::consts::TAU }

        state_delta.pitch = state_delta.pitch.clamp(-1.5, 1.5);

        transform.rotation = Quat::from_axis_angle(Vec3::Y, state_delta.yaw) * Quat::from_axis_angle(Vec3::X, state_delta.pitch);

        movable.velocity = velocity.normalize() * stats.speed;
        weapon.set_fire_state(firing);
    }
}

pub fn update_map(
    mut map_data: ResMut<crate::map::MapData>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let player_transform = player_query.get_single().unwrap();
    map_data.player_pos = player_transform.translation;
    let mut camera_transform = camera_query.get_single_mut().unwrap();
    *camera_transform = *player_transform;
}