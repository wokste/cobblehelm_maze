use bevy::{
    prelude::{Commands, Component, Entity, Query, Res, Transform, Vec3},
    time::Time,
};

use crate::map::MapData;

pub enum MapCollisionEvent {
    Stop,
    Destroy,
}

#[derive(Component)]
pub struct PhysicsBody {
    pub radius: f32,
    pub on_hit_wall: MapCollisionEvent,
}

impl PhysicsBody {
    pub fn new(radius: f32, on_hit_wall: MapCollisionEvent) -> Self {
        Self {
            radius,
            on_hit_wall,
        }
    }
}

#[derive(Component, Default)]
pub struct PhysicsMovable {
    pub velocity: Vec3,
    pub gravity: bool, // TODO: Gravity
}

impl PhysicsMovable {
    pub fn new(velocity: Vec3, gravity: bool) -> Self {
        Self { velocity, gravity }
    }
}

fn split_deltas(delta: Vec3) -> [Vec3; 2] {
    let delta_abs = delta.abs();

    if delta_abs.x > delta_abs.z {
        [Vec3::new(delta.x, 0., 0.), Vec3::new(0., 0., delta.z)]
    } else {
        [Vec3::new(0., 0., delta.z), Vec3::new(delta.x, 0., 0.)]
    }
}

// TODO: crate::grid::Grid<bool>
// TODO: Better return type
fn check_map_collision(grid_solid: &crate::grid::Grid<bool>, pos: Vec3, radius: f32) -> bool {
    let floor_height = 0.0;
    let ceil_height = 1.0;

    if pos.y - radius < floor_height {
        return true;
    }

    if pos.y + radius > ceil_height {
        return true;
    }

    let x0 = f32::floor(pos.x - radius) as i32;
    let x1 = f32::floor(pos.x + radius) as i32;
    let z0 = f32::floor(pos.z - radius) as i32;
    let z1 = f32::floor(pos.z + radius) as i32;

    for z in z0..=z1 {
        for x in x0..=x1 {
            if grid_solid[(x, z)] {
                return true;
            }
        }
    }

    false
}

pub fn do_physics(
    mut commands: Commands,
    time: Res<Time>,
    map: Res<MapData>,
    mut query: Query<(Entity, &mut Transform, &PhysicsMovable, &PhysicsBody)>,
) {
    let delta_time = time.delta_seconds();
    for (entity, mut transform, velocity, pb) in query.iter_mut() {
        if velocity.velocity.is_nan() {
            continue;
        }

        let delta = velocity.velocity * delta_time;

        let new_pos = transform.translation + delta;
        if !check_map_collision(&map.solid_map, new_pos, pb.radius) {
            transform.translation = new_pos;
        } else {
            match pb.on_hit_wall {
                MapCollisionEvent::Stop => {
                    for delta_sub in split_deltas(delta) {
                        let new_pos = transform.translation + delta_sub;
                        if !check_map_collision(&map.solid_map, new_pos, pb.radius) {
                            transform.translation = new_pos;
                        }
                    }
                }
                MapCollisionEvent::Destroy => {
                    commands
                        .entity(entity)
                        .insert(crate::lifecycle::ToBeDestroyed);
                }
            }
        }
    }
}
