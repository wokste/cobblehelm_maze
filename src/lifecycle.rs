use bevy::prelude::*;

use crate::combat::ai::AI;

#[derive(Component)]
pub struct ToBeDestroyed;

#[derive(Component)]
pub enum DestroyEffect {
    //ChangeSprite(Sprite3d),
    GameOver,
    // TODO: Add more options here
}

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
            commands.entity(entity).insert(ToBeDestroyed);
        }
    }
}

pub fn destroy_entities(
    mut commands: Commands,
    query: Query<(Entity, Option<&mut DestroyEffect>, Option<&AI>), With<ToBeDestroyed>>,
    mut game_state: ResMut<NextState<crate::game::GameState>>,
) {
    for (entity, destroy_effect, ai) in query.iter() {
        // For AI, remove the colliders from the AI map
        if let Some(ai) = ai {}

        // Destroy stuff
        if let Some(destroy_effect) = destroy_effect {
            match destroy_effect {
                DestroyEffect::GameOver => game_state.set(crate::game::GameState::GameOver),
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}
