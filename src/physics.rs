use bevy::{prelude::{Vec3, Component, Query, Res, Transform, Entity, Commands}, time::Time};

use crate::map::MapData;

pub enum MapCollisionEvent {
    Stop,
    Destroy,
}

#[derive(Component)]
pub struct PhysicsBody {
    pub velocity : Vec3,
    pub on_hit_wall : MapCollisionEvent,
    pub radius : f32,
    // TODO: Gravity
}

impl PhysicsBody {
    pub fn new(radius : f32, on_hit_wall : MapCollisionEvent) -> Self {
        Self {
            on_hit_wall,
            velocity : Vec3::ZERO,
            radius,
        }
    }

    pub fn set_velocity(mut self, velocity : Vec3) -> Self {
        self.velocity = velocity;
        self
    }
}

fn split_deltas(delta : Vec3) -> [Vec3;2] {
    let delta_abs = delta.abs();

    if delta_abs.x > delta_abs.z {
        [Vec3::new(delta.x, 0., 0.), Vec3::new(0., 0., delta.z)]
    } else {
        [Vec3::new(0., 0., delta.z), Vec3::new(delta.x, 0., 0.)]
    }
}

// TODO: crate::grid::Grid<bool>
fn check_map_collision(map : &crate::grid::Grid<crate::map::Tile>, pos : Vec3, radius : f32) -> bool {
    // TODO: Better return type
    let x0 = f32::floor(pos.x - radius) as i32;
    let x1 = f32::floor(pos.x + radius) as i32;
    let z0 = f32::floor(pos.z - radius) as i32;
    let z1 = f32::floor(pos.z + radius) as i32;

    if map[(x0, z0)].is_solid() { return true }
    if map[(x0, z1)].is_solid() { return true }
    if map[(x1, z0)].is_solid() { return true }
    if map[(x1, z1)].is_solid() { return true }

    false
}

pub fn do_physics(
    mut commands: Commands,
    time: Res<Time>,
    map: Res<MapData>,
    mut query: Query<(Entity, &mut Transform, &PhysicsBody)>,
) {
    let delta_time = time.delta_seconds();
    for (entity, mut transform, pb) in query.iter_mut() {
        if pb.velocity.is_nan() {
            continue;
        }

        let delta = pb.velocity * delta_time;

        let new_pos = transform.translation + delta;
        if !check_map_collision(&map.map, new_pos, pb.radius) {
            transform.translation = new_pos;
        } else {
            match pb.on_hit_wall {
                MapCollisionEvent::Stop => {
                    for delta_sub in split_deltas(delta)
                    {
                        let new_pos = transform.translation + delta_sub;
                        if !check_map_collision(&map.map, new_pos, pb.radius) {
                            transform.translation = new_pos;
                        }
                    }
                },
                MapCollisionEvent::Destroy => {
                    commands.entity(entity).despawn();
                },
            }
        }
    }
}