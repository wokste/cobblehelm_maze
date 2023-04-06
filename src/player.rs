use bevy::{
    input::mouse::{MouseMotion},
    prelude::*,
    //Windows,
};

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    pub keys : PlayerKeys,
    pub stats : CreatureStats,

}

#[derive(Component)]
pub struct PlayerKeys {
    pub forward: &'static [KeyCode],
    pub backward: &'static [KeyCode],
    pub left: &'static [KeyCode],
    pub right: &'static [KeyCode],
    pub rot_left: &'static [KeyCode],
    pub rot_right: &'static [KeyCode],
}

impl Default for PlayerKeys {
    fn default() -> Self {
        Self {
            forward: &[KeyCode::W],
            backward: &[KeyCode::S],
            left: &[KeyCode::A],
            right: &[KeyCode::D],
            rot_left: &[KeyCode::Left],
            rot_right: &[KeyCode::Right],
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
            rot_rate : 2.5,
            hp: 100,
            hp_max: 100,
        }
    }
}

pub fn validate_key<T>(codes: &'static [T], key: &T) -> bool
where
    T: PartialEq<T>,
{
    codes.iter().any(|m| m == key)
}

pub fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    //windows: Res<Windows>,
    mut query: Query<(&PlayerKeys, &CreatureStats, &mut Transform)>,
) {
    //let window = windows.get_primary().unwrap();
    for (key_map, stats, mut transform) in query.iter_mut() {
        let delta_time = time.delta_seconds();

        let (_, mut rotation) = transform.rotation.to_axis_angle();

        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            //if window.cursor_locked() {
                if validate_key(key_map.forward, key)   { velocity += forward }
                if validate_key(key_map.backward, key)  { velocity -= forward }
                if validate_key(key_map.left, key)      { velocity -= right }
                if validate_key(key_map.right, key)     { velocity += right }
                if validate_key(key_map.rot_left, key)  {
                    rotation += stats.rot_rate * delta_time;
                    if rotation > std::f32::consts::TAU { rotation -= std::f32::consts::TAU }
                }
                if validate_key(key_map.rot_right, key) {
                    rotation -= stats.rot_rate * delta_time;
                    if rotation < 0.0 { rotation += std::f32::consts::TAU }
                }
            //}
        }
        transform.rotation = Quat::from_rotation_y(rotation);

        velocity = velocity.normalize();

        if !velocity.is_nan() {
            transform.translation += velocity * delta_time * stats.speed
        }
    }
}