use bevy::prelude::*;

use super::{
    ai::AI,
    projectile::{spawn_projectile, ProjectileType},
    CreatureStats, DamageEvent, DamageType,
};

#[derive(Component)]
pub struct Weapon {
    firing: bool,
    cooldown: Timer,
    effect: WeaponEffect,
}

pub enum WeaponEffect {
    Ranged {
        max_dist: f32,
        accuracy: f32,
        ptype: ProjectileType,
    },
    RangedArc {
        max_dist: f32,
        arc: f32,
        ptype: ProjectileType,
        count: u8,
    },
    Melee {
        arc: f32,
        reach: f32,
        damage: i16,
        dam_type: DamageType,
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

    pub fn new_melee(fire_speed: f32, damage: i16, dam_type: DamageType) -> Self {
        Self::new(
            fire_speed,
            WeaponEffect::Melee {
                damage,
                dam_type,
                arc: 1.0,   // TODO
                reach: 1.5, // TODO
            },
        )
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
    mut query: Query<(Entity, &mut Weapon, &CreatureStats, &Transform, Option<&AI>)>,
    melee_target_query: Query<(Entity, &CreatureStats, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<crate::rendering::SpriteResource>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut ev_damage: EventWriter<DamageEvent>,
) {
    for (instigator, mut weapon, stats, transform, ai) in query.iter_mut() {
        if !weapon.cooldown.tick(time.delta()).finished() {
            continue;
        }
        if !weapon.firing {
            continue;
        }

        let pos = transform.translation;
        let dir = match ai {
            Some(ai) => ai.get_fire_dir(&pos),
            None => Some(transform.rotation * Vec3::NEG_Z),
        };

        let Some(dir) = dir else {continue;};

        match weapon.effect {
            WeaponEffect::Melee {
                damage,
                dam_type,
                arc,
                reach,
            } => {
                let mut ai_hits = false;
                for (target, target_stats, target_transform) in melee_target_query.iter() {
                    if stats.team == target_stats.team {
                        continue;
                    }

                    let delta = pos - target_transform.translation;

                    if delta.length_squared() > reach * reach {
                        continue;
                    }
                    if -delta.angle_between(dir) > arc / 2.0 {
                        continue;
                    }

                    ev_damage.send(DamageEvent {
                        instigator: Some(instigator),
                        target,
                        damage,
                        dam_type,
                    });
                    ai_hits = true;
                }

                if ai.is_some() && !ai_hits {
                    continue; // TODO: Should this be done earlier?
                }
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

        weapon.cooldown.reset();
        audio.play(asset_server.load(weapon.get_sound()));
    }
}
