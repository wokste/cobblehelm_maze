use bevy::prelude::*;

use super::{
    ai::AI,
    projectile::{spawn_projectile, ProjectileType},
    CreatureStats, Damage,
};

#[derive(Component)]
pub struct Weapon {
    firing: bool,
    cooldown: Timer,
    effect: WeaponEffect,
}

pub enum WeaponEffect {
    Ranged {
        ptype: ProjectileType,
        max_dist: f32,
        accuracy: f32,
    },
    RangedArc {
        ptype: ProjectileType,
        max_dist: f32,
        arc: f32,
        count: u8,
    },
    Melee {
        damage: Damage,
    },
}

impl Weapon {
    pub fn set_fire_state(&mut self, firing: bool) {
        self.firing = firing;

        if firing && self.cooldown.paused() {
            self.cooldown.unpause();
        }
    }

    pub fn new(fire_speed: f32, effect: WeaponEffect) -> Self {
        Self {
            cooldown: Timer::from_seconds(fire_speed, TimerMode::Once),
            firing: true,
            effect,
        }
    }

    pub fn new_melee(fire_speed: f32, damage: Damage) -> Self {
        Self::new(fire_speed, WeaponEffect::Melee { damage })
    }

    pub fn new_ranged(fire_speed: f32, projectile: ProjectileType, max_distance: f32) -> Self {
        Self::new(
            fire_speed,
            WeaponEffect::Ranged {
                ptype: projectile,
                max_dist: max_distance,
                accuracy: 0.0,
            },
        )
    }

    pub fn get_sound(&self) -> &'static str {
        fn get_projectile_sound(ptype: ProjectileType) -> &'static str {
            match ptype {
                ProjectileType::RedSpikes => "audio/shoot_redspikes.ogg",
                ProjectileType::Fire => "audio/shoot_fire.ogg",
                ProjectileType::RockLarge => "audio/shoot_rock.ogg",
                ProjectileType::RockSmall => "audio/shoot_rock.ogg",
                ProjectileType::BlueBlob => "audio/shoot_blueblob.ogg",
                ProjectileType::Shock => "audio/shoot_shock.ogg",
            }
        }

        match self.effect {
            WeaponEffect::Ranged { ptype, .. } => get_projectile_sound(ptype),
            WeaponEffect::RangedArc { ptype, .. } => get_projectile_sound(ptype),
            WeaponEffect::Melee { .. } => "audio/melee.ogg",
        }
    }
}

pub fn fire_weapons(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Weapon, &CreatureStats, &Transform, Option<&AI>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<crate::rendering::SpriteResource>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (mut weapon, stats, transform, ai) in query.iter_mut() {
        if weapon.cooldown.tick(time.delta()).finished() {
            if !weapon.firing {
                continue;
            }

            let pos = transform.translation;
            let dir = match ai {
                Some(ai) => match ai.get_fire_target() {
                    Some(taget_pos) => (taget_pos - pos).normalize(),
                    _ => {
                        continue;
                    }
                },
                None => transform.rotation * Vec3::NEG_Z,
            };

            weapon.cooldown.reset();

            match weapon.effect {
                WeaponEffect::Melee { damage } => {
                    // TODO: Implement
                }
                WeaponEffect::RangedArc {
                    ptype,
                    max_dist,
                    arc,
                    count,
                } => {
                    for i in 0..count {
                        let f = (i as f32 + 0.5) / (count as f32) - 0.5;
                        let dir = Quat::from_rotation_y(f * arc) * dir;

                        spawn_projectile(
                            ptype,
                            stats.team,
                            pos,
                            dir,
                            max_dist,
                            &mut commands,
                            &mut meshes,
                            &mut render_res,
                        );
                    }
                }
                WeaponEffect::Ranged {
                    ptype,
                    max_dist,
                    accuracy,
                } => {
                    let dir = Quat::from_rotation_y((fastrand::f32() - 0.5) * accuracy) * dir;

                    spawn_projectile(
                        ptype,
                        stats.team,
                        pos,
                        dir,
                        max_dist,
                        &mut commands,
                        &mut meshes,
                        &mut render_res,
                    );
                }
            };

            audio.play(asset_server.load(weapon.get_sound()));
        }
    }
}
