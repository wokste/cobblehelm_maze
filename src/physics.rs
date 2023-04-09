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
    // TODO: Gravity
}

impl PhysicsBody {
    pub fn new(on_hit_wall : MapCollisionEvent) -> Self {
        Self {
            on_hit_wall,
            velocity : Vec3::ZERO
        }
    }

    pub fn set_velocity(mut self, velocity : Vec3) -> Self {
        self.velocity = velocity;
        self
    }
}

fn check_map_collision(map : &crate::map::Map, pos : Vec3) -> Option<()> {
    // TODO: Better return type
    // TODO: add a radius
    if map.is_solid(f32::floor(pos.x) as i32, f32::floor(pos.z) as i32) {
        Some(())
    } else {
        None
    }
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

        let mut new_pos = transform.translation + pb.velocity * delta_time;
        if let Some(()) = check_map_collision(&map.map, new_pos) {
            match pb.on_hit_wall {
                MapCollisionEvent::Stop => {
                    new_pos = transform.translation;
                    // TODO: Check if you can wall-slide
                },
                MapCollisionEvent::Destroy => {
                    commands.entity(entity).despawn();
                },
            }
        }

        transform.translation = new_pos;
    }
}