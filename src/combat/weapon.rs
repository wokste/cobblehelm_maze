use bevy::prelude::*;

use crate::{physics::{PhysicsBody, MapCollisionEvent, PhysicsMovable}, rendering::TexCoords};

use super::{CreatureStats, Team};

#[derive(Copy, Clone)]
pub enum ProjectileType {
    RedSpikes,
    BlueBlob,
    Shock,
}

impl ProjectileType {
    fn damage(&self) -> i16 {
        match self {
            ProjectileType::BlueBlob => 15,
            ProjectileType::RedSpikes => 10,
            ProjectileType::Shock => 20,
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
    firing: FireMode,
    projectile: ProjectileType,
    cooldown: Timer,
}

impl Weapon {
    pub fn set_fire_state(&mut self, firing: FireMode) {
        self.firing = firing;

        if self.cooldown.paused() && firing != FireMode::NoFire {
            self.cooldown.unpause();
        }
    }

    pub fn new(projectile: ProjectileType, fire_speed: f32) -> Self {
        Self {
            projectile,
            cooldown: Timer::from_seconds(fire_speed, TimerMode::Once),
            firing: FireMode::NoFire,
        }
    }

    fn make_projectile(&self, team: Team) -> Projectile {
        Projectile {
            team,
            damage: self.projectile.damage(),
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    team: Team,
    damage: i16,
}

pub fn fire_weapons(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Weapon, &CreatureStats, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<crate::rendering::SpriteResource>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
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
            proto_projectile.insert(PhysicsBody::new(0.10, MapCollisionEvent::Destroy));
            proto_projectile.insert(PhysicsMovable::new(velocity, false));

            let sound = match weapon.projectile {
                ProjectileType::RedSpikes => "audio/shoot_redspikes.ogg",
                ProjectileType::BlueBlob => "audio/shoot_blueblob.ogg",
                ProjectileType::Shock => "audio/shoot_shock.ogg",
            };
            audio.play(asset_server.load(sound));
        }
    }
}

pub fn check_projectile_creature_collisions(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &PhysicsBody, &Transform)>,
    mut target_query: Query<(Entity, &PhysicsBody, &mut CreatureStats, &Transform)>,
    mut game: ResMut<crate::GameInfo>,
    mut game_state: ResMut<NextState<crate::game::GameState>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (projectile_entity, projectile, projectile_body, projectile_transform) in projectile_query.iter_mut() {
        for (target_entity, target_body, mut stats, target_transform) in target_query.iter_mut() {
            if projectile.team == stats.team {
                continue;
            }
            
            let distance = projectile_body.radius + target_body.radius;
            if projectile_transform.translation.distance_squared(target_transform.translation) > distance * distance {
                continue;
            }

            stats.hp -= projectile.damage;
            if stats.team == Team::Players {
                game.hp_perc = f32::clamp((stats.hp as f32) / (stats.hp_max as f32), 0.0, 1.0);
                let sound = asset_server.load("audio/player_hurt.ogg");
                audio.play(sound);
            } else {
                let sound = asset_server.load("audio/monster_hurt.ogg");
                audio.play(sound);
            }


            if stats.hp <= 0 {
                if stats.team == Team::Players {
                    game_state.set(crate::game::GameState::GameOver);
                } else {
                    commands.entity(target_entity).despawn();
                    game.score += 50; // TODO: What kind of score to use?
                    game.coins += 1; // TODO: This should be a pickup
                }
            }
            
            commands.entity(projectile_entity).despawn();
        }
    }
}