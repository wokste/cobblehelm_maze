use bevy::{prelude::{Vec3, Component, Query, Res, Transform}, time::Time};

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
}

pub fn do_physics(
    time: Res<Time>,
//    map: Res<MapData>, // TODO
    mut query: Query<(&mut Transform, &PhysicsBody)>,
) {
    let delta_time = time.delta_seconds();
    for (mut transform, pb) in query.iter_mut() {
        if !pb.velocity.is_nan() {
            transform.translation += pb.velocity * delta_time
        }

        // TODO: Implement collisions
    }
    
}