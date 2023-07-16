use bevy::prelude::*;

#[derive(Component)]
pub struct LevelObject;

#[derive(Component)]
pub struct Ttl {
    timer: Timer,
}

impl Ttl {
    pub fn new(duration: f32) -> Self {
        Ttl {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

pub fn check_ttl(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Ttl)>) {
    for (entity, mut ttl) in query.iter_mut() {
        if ttl.timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}
