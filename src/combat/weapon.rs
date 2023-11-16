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
    pub effect: WeaponEffect,
    pub damage: i16,
    pub dam_type: DamageType,

    /// For melee, this is the reach, for range weapons, it is the max distance that projectiles can fly
    pub range: f32,
}

pub enum WeaponEffect {
    Ranged {
        ptype: ProjectileType,
        accuracy: f32,
    },
    RangedArc {
        ptype: ProjectileType,
        arc: f32,
        count: u8,
    },
    Melee {
        arc: f32,
    },
}

impl Weapon {
    pub fn set_fire_state(&mut self, firing: bool) {
        self.firing = firing;

        if firing && self.cooldown.paused() {
            self.cooldown.unpause();
        }
    }

    pub fn new(
        fire_speed: f32,
        damage: i16,
        dam_type: DamageType,
        range: f32,
        effect: WeaponEffect,
    ) -> Self {
        Self {
            cooldown: Timer::from_seconds(fire_speed, TimerMode::Once),
            firing: true,
            effect,
            damage,
            range,
            dam_type,
        }
    }

    pub fn new_melee(fire_speed: f32, damage: i16, dam_type: DamageType) -> Self {
        Self::new(
            fire_speed,
            damage,
            dam_type,
            1.5, // TODO
            WeaponEffect::Melee {
                arc: 1.0,   // TODO
            },
        )
    }

    pub fn new_ranged(
        fire_speed: f32,
        projectile: ProjectileType,
        range: f32,
        damage: i16,
        dam_type: DamageType,
    ) -> Self {
        Self::new(
            fire_speed,
            damage,
            dam_type,
            range,
            WeaponEffect::Ranged {
                ptype: projectile,
                accuracy: 0.0,
            },
        )
    }

    pub fn get_sound(&self) -> &'static str {
        fn get_projectile_sound(ptype: ProjectileType) -> &'static str {
            match ptype {
                ProjectileType::RedSpikes => "audio/shoot_redspikes.ogg",
                ProjectileType::Fire => "audio/shoot_fire.ogg",
                ProjectileType::Rock => "audio/shoot_rock.ogg",
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
    mut render_res: ResMut<crate::render::RenderResource>,
    asset_server: Res<AssetServer>,
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
            Some(ai) => match ai.state() {
                super::ai::AIState::SeePlayer(player_pos) => (*player_pos - pos).normalize(),
                _ => {
                    continue;
                }
            },
            None => transform.rotation * Vec3::NEG_Z,
        };

        match weapon.effect {
            WeaponEffect::Melee { arc } => {
                let mut ai_hits = false;
                for (target, target_stats, target_transform) in melee_target_query.iter() {
                    if stats.team == target_stats.team {
                        continue;
                    }

                    let delta = pos - target_transform.translation;

                    if delta.length_squared() > weapon.range * weapon.range {
                        continue;
                    }
                    if -delta.angle_between(dir) > arc / 2.0 {
                        continue;
                    }

                    ev_damage.send(DamageEvent {
                        instigator: Some(instigator),
                        target,
                        damage: weapon.damage,
                        dam_type: weapon.dam_type,
                    });
                    ai_hits = true;
                }

                if ai.is_some() && !ai_hits {
                    continue; // Ignore the attack since the AI would not have attacked. Safe because no events would have been send.
                }
            }
            WeaponEffect::RangedArc { ptype, arc, count } => {
                for i in 0..count {
                    let f = (i as f32 + 0.5) / (count as f32) - 0.5;
                    let dir = Quat::from_rotation_y(f * arc) * dir;

                    spawn_projectile(
                        instigator,
                        stats.team,
                        pos,
                        dir,
                        &weapon,
                        ptype,
                        &mut commands,
                        &mut meshes,
                        &mut render_res,
                    );
                }
            }
            WeaponEffect::Ranged { ptype, accuracy } => {
                let dir = Quat::from_rotation_y((fastrand::f32() - 0.5) * accuracy) * dir;

                spawn_projectile(
                    instigator,
                    stats.team,
                    pos,
                    dir,
                    &weapon,
                    ptype,
                    &mut commands,
                    &mut meshes,
                    &mut render_res,
                );
            }
        };

        weapon.cooldown.reset();
        commands.spawn(AudioBundle {
            source: asset_server.load(weapon.get_sound()),
            settings: default(),
        });
    }
}
