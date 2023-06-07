use bevy::prelude::*;

use crate::{combat::{CreatureStats, Team}, physics::{PhysicsBody, MapCollisionEvent}, rendering::TexCoords};

#[derive(Copy, Clone)]
pub enum ProjectileType {
    RedSpikes,
    BlueBlob,
    Shock,
}

impl ProjectileType {
    fn damage(&self) -> i16 {
        match self {
            ProjectileType::BlueBlob => 3,
            ProjectileType::RedSpikes => 1,
            ProjectileType::Shock => 4,
        }
    }

    fn speed(&self) -> f32 {
        match self {
            ProjectileType::BlueBlob => 8.0,
            ProjectileType::RedSpikes => 6.0,
            ProjectileType::Shock => 2.0,
        }
    }

    fn make_uv(&self) -> TexCoords {
        match self {
            ProjectileType::RedSpikes => TexCoords::new(0..1, 6),
            ProjectileType::BlueBlob => TexCoords::new(1..2, 6),
            ProjectileType::Shock => TexCoords::new(2..5, 6),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum FireMode {
    NoFire,
    Fire,
    FireAt(Vec3),
}

#[derive(Component)]
pub struct Weapon {
    firing : FireMode,
    projectile : ProjectileType,
    cooldown : Timer,
}

impl Weapon {
    pub fn set_fire_state(&mut self, firing : FireMode) {
        self.firing = firing;

        if self.cooldown.paused() && firing != FireMode::NoFire {
            self.cooldown.unpause();
        }
    }

    pub fn new(projectile : ProjectileType, fire_speed: f32) -> Self {
        Self {
            projectile,
            cooldown : Timer::from_seconds(fire_speed, TimerMode::Once),
            firing : FireMode::NoFire,
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
    time: Res<Time>,
    mut query: Query<(&mut Weapon, &CreatureStats, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res : ResMut<crate::rendering::SpriteResource>,
) {
    for (mut weapon, stats, transform) in query.iter_mut() {
        if weapon.cooldown.tick(time.delta()).finished() {
            let direction = match weapon.firing {
                FireMode::NoFire => { continue }
                FireMode::Fire => transform.rotation * Vec3::NEG_Z,
                FireMode::FireAt(target_pos) => {(target_pos - transform.translation).normalize()},
            };
            weapon.cooldown.reset();

            let velocity = direction * weapon.projectile.speed();

            let uv = weapon.projectile.make_uv();

            let mut proto_projectile = commands.spawn(uv.to_sprite_bundle(transform.translation, 0.1, &mut meshes, &mut render_res));
            proto_projectile.insert(crate::rendering::FaceCamera);
            proto_projectile.insert(weapon.make_projectile(stats.team));
            proto_projectile.insert(PhysicsBody::new(MapCollisionEvent::Destroy).set_velocity( velocity ));
        }
    }
}

pub fn check_projectile_creature_collisions(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &Transform)>,
    mut target_query: Query<(Entity, &mut CreatureStats, &Transform)>,
    mut hud: ResMut<crate::hud::HUDInfo>,
) {
    for (projectile_entity, projectile, projectile_transform) in projectile_query.iter_mut() {
        for (monster_entity, mut stats, target_transform) in target_query.iter_mut() {
            if projectile.team == stats.team {
                continue;
            }
            
            if projectile_transform.translation.distance_squared(target_transform.translation) > 1.0 { // TODO: Projectile and monster radius
                continue;
            }

            stats.hp -= projectile.damage;

            if stats.hp <= 0 {
                if stats.team == Team::Players {
                    // TODO: Game over
                } else {
                    commands.entity(monster_entity).despawn();
                    hud.score_points(10); // TODO: What kind of score to use?
                }
            }
            
            commands.entity(projectile_entity).despawn();
        }
    }
}