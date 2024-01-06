use bevy::{
    prelude::{Commands, Component, Entity, Query, Res, Transform, Vec3},
    time::Time,
};

use crate::map::MapData;

#[derive(Clone, Copy, PartialEq)]
pub enum MapCollisionEvent {
    #[allow(dead_code)] // TODO: Remove after 0.2
    Bounce(f32),
    Destroy,
    Stop,
}

#[derive(Component, Clone)]
pub struct Collider {
    pub pos: Vec3,
    pub radius: f32,
}

impl Collider {
    pub fn new(pos: Vec3, radius: f32) -> Self {
        Self { pos, radius }
    }

    pub fn with_pos<'a>(&self, pos: Vec3) -> Self {
        let mut clone = self.clone();
        clone.pos = pos;
        clone
    }

    pub fn collide_other(&self, other: &Self) -> bool {
        let xz_dist = self.radius + other.radius;
        let xz_dist_squared = xz_dist * xz_dist;

        self.pos.distance_squared(other.pos) <= xz_dist_squared
    }

    // TODO: Better return type
    fn collide_map(&self, grid_solid: &crate::grid::Grid<bool>) -> bool {
        /*
        TODO: Ceiling and floor heights
        let floor_height = 0.0;
        let ceil_height = 1.0;

        if pos.y - radius < floor_height {
            return true;
        }

        if pos.y + radius > ceil_height {
            return true;
        }
        */

        let x0 = f32::floor(self.pos.x - self.radius) as i32;
        let x1 = f32::floor(self.pos.x + self.radius) as i32;
        let z0 = f32::floor(self.pos.z - self.radius) as i32;
        let z1 = f32::floor(self.pos.z + self.radius) as i32;

        for z in z0..=z1 {
            for x in x0..=x1 {
                if grid_solid[(x, z)] {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Component)]
pub struct PhysicsMovable {
    pub velocity: Vec3,
    pub on_hit_wall: MapCollisionEvent,
}

impl PhysicsMovable {
    pub fn new(velocity: Vec3, on_hit_wall: MapCollisionEvent) -> Self {
        Self {
            velocity,
            on_hit_wall,
        }
    }

    fn velocity_axis(&self) -> [Vec3; 2] {
        let vel = self.velocity;

        if vel.x.abs() > vel.z.abs() {
            [Vec3::new(vel.x, 0., 0.), Vec3::new(0., 0., vel.z)]
        } else {
            [Vec3::new(0., 0., vel.z), Vec3::new(vel.x, 0., 0.)]
        }
    }

    fn move_bounce(&mut self, pb: &mut Collider, dt: f32, map: &MapData) {
        let mut new_velocity = Vec3::ZERO;

        for axis in self.velocity_axis() {
            let new_pos = pb.pos + (axis * dt);
            if !pb.with_pos(new_pos).collide_map(&map.solid_map) {
                pb.pos = new_pos;
                new_velocity += axis;
                continue;
            }
            if let MapCollisionEvent::Bounce(bounce) = self.on_hit_wall {
                let new_pos = pb.pos + (axis * dt * -bounce);
                if !pb.with_pos(new_pos).collide_map(&map.solid_map) {
                    pb.pos = new_pos;
                    new_velocity += axis;
                    continue;
                }
            }
        }
        self.velocity = new_velocity;
    }
}

pub fn do_physics(
    mut commands: Commands,
    time: Res<Time>,
    map: Res<MapData>,
    mut query: Query<(Entity, &mut Transform, &mut PhysicsMovable, &mut Collider)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut transform, mut movable, mut pb) in query.iter_mut() {
        if movable.velocity.is_nan() {
            continue;
        }

        let delta = movable.velocity * dt;

        let new_pos = transform.translation + delta;
        if !pb.with_pos(new_pos).collide_map(&map.solid_map) {
            pb.pos = new_pos;
        } else {
            match movable.on_hit_wall {
                MapCollisionEvent::Bounce(_) | MapCollisionEvent::Stop => {
                    movable.move_bounce(&mut pb, dt, &map);
                }
                MapCollisionEvent::Destroy => commands.entity(entity).despawn(),
            }
        }

        transform.translation = pb.pos;
    }
}
