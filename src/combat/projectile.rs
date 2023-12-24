use bevy::prelude::*;

use crate::{
    physics::{MapCollisionEvent, PhysicsBody, PhysicsMovable},
    render::{spritemap::SpriteSeq, RenderResource},
};

use super::{ai::AiMover, weapon::Weapon, CreatureStats, DamageEvent, DamageType, Team};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ProjectileType {
    RedSpikes,
    BlueBlob,
    Shock,
    Rock,
    Fire,
    Snowball,
}

impl ProjectileType {
    pub fn speed(&self) -> f32 {
        match self {
            ProjectileType::BlueBlob => 8.0,
            ProjectileType::RedSpikes => 6.0,
            ProjectileType::Rock => 6.0,
            ProjectileType::Fire => 6.0,
            ProjectileType::Shock => 2.0,
            ProjectileType::Snowball => 6.0,
        }
    }

    pub fn make_uv(&self, tiles: &crate::render::spritemap::SpriteMap) -> SpriteSeq {
        let str = match self {
            ProjectileType::RedSpikes => "red_spikes.png",
            ProjectileType::BlueBlob => "blue_blob.png",
            ProjectileType::Shock => "shock.png",
            ProjectileType::Rock => "rock.png",
            ProjectileType::Fire => "fire.png",
            ProjectileType::Snowball => "snowball.png",
        };
        tiles.get_projectile(str)
    }

    fn make_projectile(&self, team: Team, instigator: Entity, weapon: &Weapon) -> Projectile {
        Projectile {
            team,
            damage: weapon.damage,
            dam_type: weapon.dam_type,
            instigator,
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub team: Team,
    pub damage: i16,
    pub dam_type: DamageType,
    pub instigator: Entity,
}

pub fn spawn_projectile(
    instigator: Entity,
    team: Team,
    pos: Vec3,
    dir: Vec3,
    weapon: &Weapon,
    ptype: ProjectileType,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    render_res: &mut ResMut<RenderResource>,
) {
    let velocity = dir * ptype.speed();

    let uv = ptype.make_uv(&render_res.sprites);

    let mut proto_projectile = commands.spawn(uv.to_sprite_bundle(pos, meshes, render_res));
    proto_projectile.insert(crate::render::Animation::new(uv, 0.1));
    proto_projectile.insert(ptype.make_projectile(team, instigator, weapon));
    proto_projectile.insert(PhysicsBody::new(0.10, MapCollisionEvent::Destroy)); // TODO: Electricity should have a higher radius.
    proto_projectile.insert(PhysicsMovable::new(velocity));

    if weapon.range.is_finite() {
        proto_projectile.insert(crate::lifecycle::Ttl::new(weapon.range / ptype.speed()));
    }
}

pub fn check_collisions(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &PhysicsBody, &Transform)>,
    mut target_query: Query<(Entity, &PhysicsBody, &CreatureStats, &Transform)>,
    mut ev_damage: EventWriter<DamageEvent>,
) {
    for (projectile_entity, projectile, projectile_body, projectile_transform) in
        projectile_query.iter_mut()
    {
        for (target_entity, target_body, stats, target_transform) in target_query.iter_mut() {
            if projectile.team == stats.team {
                continue;
            }

            let distance = projectile_body.radius + target_body.radius;
            if projectile_transform
                .translation
                .distance_squared(target_transform.translation)
                > distance * distance
            {
                continue;
            }

            ev_damage.send(DamageEvent {
                instigator: Some(projectile.instigator),
                target: target_entity,
                damage: projectile.damage,
                dam_type: projectile.dam_type,
            });

            commands.entity(projectile_entity).despawn();
        }
    }
}

pub fn take_damage_system(
    mut commands: Commands,
    mut target_query: Query<(&mut CreatureStats, Option<&mut AiMover>)>,
    mut game: ResMut<crate::GameInfo>,
    mut game_state: ResMut<NextState<crate::game::GameState>>,
    mut map_data: ResMut<crate::map::MapData>,
    asset_server: Res<AssetServer>,
    mut ev_damage: EventReader<DamageEvent>,
    mut menu_info: ResMut<crate::ui::menus::MenuInfo>,
) {
    for ev in ev_damage.read() {
        let Ok((mut stats, mut ai_pos)) =
            target_query.get_mut(ev.target) else {continue;};

        let hurt = stats.take_damage(
            ev,
            &mut commands,
            &mut game,
            &mut game_state,
            &mut map_data,
            &mut menu_info,
            ai_pos.as_deref_mut(),
        );

        if hurt {
            if let Some(source) = stats.get_hurt_sound(&asset_server) {
                commands.spawn(AudioBundle {
                    source,
                    settings: default(),
                });
            }
        }
    }
}
