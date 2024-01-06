use bevy::{
    prelude::{Commands, Component, Entity, Query, Res, Transform, Vec3},
    time::Time,
};

use crate::map::MapData;

pub enum MapCollisionEvent {
    Bounce(f32),
    Destroy,
    Stop,
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
}

impl PhysicsMovable {
    pub fn new(velocity: Vec3) -> Self {
        Self { velocity }
    }

    fn velocity_axis(&self) -> [Vec3; 2] {
        let vel = self.velocity;

        if vel.x.abs() > vel.z.abs() {
            [Vec3::new(vel.x, 0., 0.), Vec3::new(0., 0., vel.z)]
        } else {
            [Vec3::new(0., 0., vel.z), Vec3::new(vel.x, 0., 0.)]
        }
    }

    fn move_bounce(&mut self, pos: &mut Vec3, dt: f32, map: &MapData, pb: &PhysicsBody) {
        let mut new_velocity = Vec3::ZERO;

        for axis in self.velocity_axis() {
            let new_pos = *pos + (axis * dt);
            if !check_map_collision(&map.solid_map, new_pos, pb.radius) {
                *pos = new_pos;
                new_velocity += axis;
                continue;
            }
            if let MapCollisionEvent::Bounce(bounce) = pb.on_hit_wall {
                let new_pos = *pos + (axis * dt * -bounce);
                if !check_map_collision(&map.solid_map, new_pos, pb.radius) {
                    *pos = new_pos;
                    new_velocity += axis;
                    continue;
                }
            }
        }
        self.velocity = new_velocity;
    }
}

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
    mut query: Query<(Entity, &mut Transform, &mut PhysicsMovable, &PhysicsBody)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut transform, mut movable, pb) in query.iter_mut() {
        if movable.velocity.is_nan() {
            continue;
        }

        let delta = movable.velocity * dt;

        let new_pos = transform.translation + delta;
        if !check_map_collision(&map.solid_map, new_pos, pb.radius) {
            transform.translation = new_pos;
        } else {
            match pb.on_hit_wall {
                MapCollisionEvent::Bounce(_) | MapCollisionEvent::Stop => {
                    movable.move_bounce(&mut transform.translation, dt, &map, pb);
                }
                MapCollisionEvent::Destroy => commands.entity(entity).despawn(),
            }
        }
    }
}
