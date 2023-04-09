use bevy::{
    prelude::{Component, Query, Res, Transform, Entity, Commands},
    time::{Time, Timer, TimerMode},
};

use crate::player::CreatureStats;

#[derive(Eq, PartialEq)]
#[derive(Component)]
pub enum Team {
    Players,
//    Monsters,
//    Environment,
}

pub enum ProjectileType {
//    Fireball,
    BlueThing,
}

#[derive(Component)]
pub struct Weapon {
    firing : bool,
    projectile : ProjectileType,
    cooldown : Timer,
}

impl Weapon {
    pub fn new(projectile : ProjectileType) -> Self {
        Self {
            projectile,
            cooldown : Timer::from_seconds(0.7, TimerMode::Repeating), // TODO: make time configurable
            firing : false,
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    spawn_type : ProjectileType,
    team : Team,
    damage : i16,
}

pub fn fire_weapons(
    time: Res<Time>,
    mut query: Query<(&mut Weapon, &Transform)>,
) {
    for (mut weapon, transform) in query.iter_mut() {
        if weapon.firing && weapon.cooldown.tick(time.delta()).just_finished() {
            println!("fire");
        }
        // TODO: Implement
    }
}

pub fn check_projectile_creature_collisions(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &Transform)>,
    mut target_query: Query<(&Team, &mut CreatureStats, &Transform)>,
) {
    for (projectile_entity, projectile, projectile_transform) in projectile_query.iter_mut() {
        for (target_team, mut stats, target_transform) in target_query.iter_mut() {
            if projectile.team == *target_team {
                continue;
            }
            
            if projectile_transform.translation.distance_squared(target_transform.translation) > 1.0 { // TODO: Distance
                continue;
            }

            commands.entity(projectile_entity).despawn();
            stats.hp -= projectile.damage;
        }
    }
}