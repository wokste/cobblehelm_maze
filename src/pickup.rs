use bevy::prelude::*;

use crate::{combat::{CreatureStats, player::Player}, GameInfo, physics::PhysicsBody};

#[derive(Component)]
pub enum Pickup {
    Health(i16),
    NextLevel,
    Coins(i32),
    //Key(u8),
    //Ammo(u8,i16),
}

impl Pickup {
    fn can_take(&self, stats: &CreatureStats) -> bool {
        match self {
            Pickup::Health(_) => stats.hp < stats.hp_max,
            _=> true,
        }
    }

    fn take(&self, game_info: &mut GameInfo, stats: &mut CreatureStats, game_state: &mut ResMut<NextState<crate::game::GameState>>) {
        match self {
            Pickup::Health(gain) => {
                stats.hp = i16::min(stats.hp+gain, stats.hp_max);
            },
            Pickup::NextLevel => {
                game_state.set(crate::game::GameState::NextLevel);
            },
            Pickup::Coins(gain) => {
                game_info.coins += gain;
            },
        }
    }
}


pub fn check_pickups(
    mut commands: Commands,
    mut player_query: Query<(&PhysicsBody, &mut CreatureStats, &Transform), With<Player>>,
    mut pickup_query: Query<(Entity, &Pickup, &PhysicsBody, &Transform)>,
    mut game: ResMut<crate::GameInfo>,
    mut game_state: ResMut<NextState<crate::game::GameState>>,
) {
    for (player_body, mut stats, player_transform) in player_query.iter_mut() {
        for (pickup_entity, pickup, pickup_body, pickup_transform) in pickup_query.iter_mut() {
            let distance = pickup_body.radius + player_body.radius;
            if pickup_transform.translation.distance_squared(player_transform.translation) > distance * distance {
                continue;
            }

            if pickup.can_take(&stats) {
                pickup.take(&mut game, &mut stats, &mut game_state);
                commands.entity(pickup_entity).despawn();
            }
        }
    }
}