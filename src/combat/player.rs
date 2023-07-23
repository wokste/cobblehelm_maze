use bevy::{ecs::event::ManualEventReader, input::mouse::MouseMotion, prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::physics::{MapCollisionEvent, PhysicsBody, PhysicsMovable};

use super::{
    weapon::{FireMode, ProjectileType, Weapon},
    CreatureStats,
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
            player: Player {},
            stats: CreatureStats::player(),
            physisc: PhysicsBody::new(0.125, MapCollisionEvent::Stop),
            weapon: Weapon::new(ProjectileType::BlueBlob, 0.3, 12.0),
            velocity: PhysicsMovable::default(),
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    #[default]
    None,
    Forward,
    Backward,
    Left,
    Right,
    RotLeft,
    RotRight,
    Fire,
    Interact,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct InputMap {
    pub keys: HashMap<KeyCode, InputAction>,
    pub mouse_buttons: HashMap<MouseButton, InputAction>,

    pub rot_rate_key: f32,
    pub rot_rate_mouse: f32,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            keys: HashMap::from([
                (KeyCode::W, InputAction::Forward),
                (KeyCode::S, InputAction::Backward),
                (KeyCode::A, InputAction::Left),
                (KeyCode::D, InputAction::Right),
                (KeyCode::Left, InputAction::RotLeft),
                (KeyCode::Right, InputAction::RotRight),
                (KeyCode::LControl, InputAction::Fire),
                (KeyCode::RControl, InputAction::Fire),
                (KeyCode::Space, InputAction::Interact),
            ]),
            mouse_buttons: HashMap::from([
                (MouseButton::Left, InputAction::Fire),
                (MouseButton::Right, InputAction::Interact),
            ]),
            rot_rate_key: 2.5,
            rot_rate_mouse: 0.1,
        }
    }
}

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

impl InputState {
    fn clamp_mouse(&mut self) {
        if self.yaw > std::f32::consts::TAU {
            self.yaw -= std::f32::consts::TAU
        }
        if self.yaw < 0.0 {
            self.yaw += std::f32::consts::TAU
        }

        self.pitch = self.pitch.clamp(-1.5, 1.5);
    }

    fn to_quat(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, self.yaw) * Quat::from_axis_angle(Vec3::X, self.pitch)
    }
}

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mouse_motion: Res<Events<MouseMotion>>,
    mut state: ResMut<InputState>,
    time: Res<Time>,
    mut query: Query<
        (
            &CreatureStats,
            &mut Transform,
            &mut PhysicsMovable,
            &mut Weapon,
        ),
        With<Player>,
    >,
    key_map: Res<InputMap>,
) {
    let delta_time = time.delta_seconds();
    let mut state_delta = state.as_mut();
    for (stats, mut transform, mut movable, mut weapon) in query.iter_mut() {
        let mut firing = FireMode::NoFire;
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for ev in state_delta.reader_motion.iter(&mouse_motion) {
            state_delta.pitch -= (key_map.rot_rate_mouse * ev.delta.y).to_radians();
            state_delta.yaw -= (key_map.rot_rate_mouse * ev.delta.x).to_radians();
        }

        let mut acts = tinyvec::tiny_vec!([InputAction; 4]);
        for key in keys.get_pressed() {
            if let Some(act) = key_map.keys.get(key) {
                acts.push(*act);
            }
        }

        for button in mouse.get_pressed() {
            if let Some(act) = key_map.mouse_buttons.get(button) {
                acts.push(*act);
            }
        }

        for act in acts {
            match act {
                InputAction::None => {}
                InputAction::Forward => velocity += forward,
                InputAction::Backward => velocity -= forward,
                InputAction::Left => velocity -= right,
                InputAction::Right => velocity += right,
                InputAction::RotLeft => state_delta.yaw += key_map.rot_rate_key * delta_time,
                InputAction::RotRight => state_delta.yaw -= key_map.rot_rate_key * delta_time,
                InputAction::Fire => firing = FireMode::Fire,
                InputAction::Interact => {
                    // TODO: Do interaction
                }
            };
        }

        state_delta.clamp_mouse();
        transform.rotation = state_delta.to_quat();
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
    map_data.player_pos = *player_transform;
    let mut camera_transform = camera_query.get_single_mut().unwrap();
    *camera_transform = *player_transform;
}
