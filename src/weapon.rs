use bevy::prelude::*;

use crate::{player::CreatureStats, physics::{PhysicsBody, MapCollisionEvent}};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Team {
    Players,
    Monsters,
//    Environment,
}

#[derive(Copy, Clone)]
pub enum ProjectileType {
    Fireball,
    BlueThing,
}

impl ProjectileType {
    fn damage(&self) -> i16 {
        match self {
            ProjectileType::BlueThing => 2,
            ProjectileType::Fireball => 3,
        }
    }

    fn speed(&self) -> f32 {
        match self {
            ProjectileType::BlueThing => 8.0,
            ProjectileType::Fireball => 5.0,
        }
    }

    fn fire_speed(&self) -> f32 {
        match self {
            ProjectileType::BlueThing => 0.3,
            ProjectileType::Fireball => 0.6,
        }
    }

    fn to_color(&self) -> Color {
        match self {
            ProjectileType::BlueThing => Color::CYAN,
            ProjectileType::Fireball => Color::ORANGE_RED,

        }
    }
}

#[derive(Component)]
pub struct Weapon {
    pub firing : bool,
    projectile : ProjectileType,
    cooldown : Timer,
}

impl Weapon {
    pub fn new(projectile : ProjectileType) -> Self {
        Self {
            projectile,
            cooldown : Timer::from_seconds(projectile.fire_speed(), TimerMode::Repeating), // TODO: make time configurable
            firing : false,
        }
    }

    fn make_projectile(&self, team : Team) -> Projectile {
        Projectile {
            team: team,
            damage: self.projectile.damage(),
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    team : Team,
    damage : i16,
}

pub fn fire_weapons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut query: Query<(&mut Weapon, &CreatureStats, &Transform)>,
) {
    for (mut weapon, stats, transform) in query.iter_mut() {
        if weapon.firing && weapon.cooldown.tick(time.delta()).just_finished() {
            let mut proto_projectile = commands.spawn(PbrBundle {
                mesh: meshes.add( Mesh::from(shape::Cube{ size: 0.2 })),
                material: materials.add(StandardMaterial {
                    base_color: weapon.projectile.to_color(),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_translation(transform.translation),
                ..default()
            });
            
            proto_projectile.insert(weapon.make_projectile(stats.team));
            let velocity = transform.rotation * Vec3::NEG_Z * weapon.projectile.speed();
            proto_projectile.insert(PhysicsBody::new(MapCollisionEvent::Destroy).set_velocity( velocity ));
        }
    }
}

pub fn check_projectile_creature_collisions(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &Transform)>,
    mut target_query: Query<(&mut CreatureStats, &Transform)>,
) {
    for (projectile_entity, projectile, projectile_transform) in projectile_query.iter_mut() {
        for (mut stats, target_transform) in target_query.iter_mut() {
            if projectile.team == stats.team {
                continue;
            }
            
            if projectile_transform.translation.distance_squared(target_transform.translation) > 1.0 { // TODO: Distance
                continue;
            }

            println!("Boom");
            commands.entity(projectile_entity).despawn();
            stats.hp -= projectile.damage;
        }
    }
}