use bevy::{
    ecs::event::ManualEventReader,
    input::{
        gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadEvent},
        mouse::MouseMotion,
    },
    prelude::*,
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

use crate::physics::{MapCollisionEvent, PhysicsBody, PhysicsMovable};

use super::{projectile::ProjectileType, weapon::Weapon, CreatureStats};

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
        let mut weapon = Weapon::new_ranged(0.3, ProjectileType::BlueBlob, 12.0);
        weapon.set_fire_state(false); // Unlike AI's don't automatically fire.

        Self {
            player: Player {},
            stats: CreatureStats::player(),
            physisc: PhysicsBody::new(0.125, MapCollisionEvent::Stop),
            weapon,
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
    Pause,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct InputMap {
    pub keys: HashMap<KeyCode, InputAction>,
    pub button_rot_rate: f32,

    pub mouse_buttons: HashMap<MouseButton, InputAction>,
    pub mouse_rot_rate: f32,

    pub pad_buttons: HashMap<GamepadButtonType, InputAction>,
    pub pad_rot_x: GamepadAxisType,
    pub pad_rot_y: GamepadAxisType,
    pub pad_rot_rate: f32,
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
                (KeyCode::Escape, InputAction::Pause),
            ]),
            button_rot_rate: 2.5,
            mouse_buttons: HashMap::from([
                (MouseButton::Left, InputAction::Fire),
                (MouseButton::Right, InputAction::Interact),
            ]),
            mouse_rot_rate: 0.1,
            pad_buttons: HashMap::from([
                (GamepadButtonType::LeftTrigger, InputAction::Fire),
                (GamepadButtonType::RightTrigger, InputAction::Fire),
                (GamepadButtonType::South, InputAction::Interact), // A
                (GamepadButtonType::DPadUp, InputAction::Forward),
                (GamepadButtonType::DPadDown, InputAction::Backward),
                (GamepadButtonType::DPadLeft, InputAction::Left),
                (GamepadButtonType::DPadRight, InputAction::Right),
                (GamepadButtonType::Select, InputAction::Pause),
            ]),
            pad_rot_x: GamepadAxisType::RightStickX,
            pad_rot_y: GamepadAxisType::RightStickY,
            pad_rot_rate: 0.1,
        }
    }
}

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
    gamepad: Option<Gamepad>,
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

pub fn gamepad_connections(
    mut state: ResMut<InputState>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.iter() {
        if let GamepadEvent::Connection(GamepadConnectionEvent {
            gamepad,
            connection,
        }) = ev
        {
            match connection {
                GamepadConnection::Connected(info) => {
                    println!("New gamepad connected with name: {}", info.name);

                    if state.gamepad.is_none() {
                        state.gamepad = Some(*gamepad);
                    }
                }
                GamepadConnection::Disconnected => {
                    println!("Lost gamepad connection");

                    if state.gamepad == Some(*gamepad) {
                        state.gamepad = None;
                    }
                }
            }
        }
    }
}

pub fn get_player_input(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mouse_motion: Res<Events<MouseMotion>>,
    mut state: ResMut<InputState>,
    key_map: Res<InputMap>,
    pad_axes: Res<Axis<GamepadAxis>>,
    pad_buttons: Res<Input<GamepadButton>>,
) -> tinyvec::TinyVec<[InputAction; 4]> {
    let mut state = state.as_mut();

    for ev in state.reader_motion.iter(&mouse_motion) {
        state.pitch -= (key_map.mouse_rot_rate * ev.delta.y).to_radians();
        state.yaw -= (key_map.mouse_rot_rate * ev.delta.x).to_radians();
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

    if let Some(gamepad) = state.gamepad {
        fn axis(gamepad: Gamepad, axis_type: GamepadAxisType) -> GamepadAxis {
            GamepadAxis { gamepad, axis_type }
        }

        // TODO: Configurable sticks
        if let Some(dx) = pad_axes.get(axis(gamepad, key_map.pad_rot_x)) {
            state.pitch += dx;
        }
        if let Some(dy) = pad_axes.get(axis(gamepad, key_map.pad_rot_y)) {
            state.pitch += dy;
        }

        for (button_type, action) in key_map.pad_buttons.iter() {
            let button = GamepadButton {
                gamepad,
                button_type: *button_type,
            };
            if pad_buttons.pressed(button) {
                acts.push(*action);
            }
        }
    };

    acts
}

pub fn handle_player_input(
    acts: In<tinyvec::TinyVec<[InputAction; 4]>>,
    mut game_state: ResMut<NextState<crate::game::GameState>>,
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
        let mut firing = false;
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for act in &acts.0 {
            match act {
                InputAction::None => {}
                InputAction::Forward => velocity += forward,
                InputAction::Backward => velocity -= forward,
                InputAction::Left => velocity -= right,
                InputAction::Right => velocity += right,
                InputAction::RotLeft => state_delta.yaw += key_map.button_rot_rate * delta_time,
                InputAction::RotRight => state_delta.yaw -= key_map.button_rot_rate * delta_time,
                InputAction::Fire => firing = true,
                InputAction::Interact => {
                    // TODO: Do interaction
                }
                InputAction::Pause => {
                    game_state.set(crate::game::GameState::Paused);
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
