use bevy::prelude::*;

use crate::{
    physics::{MapCollisionEvent, PhysicsBody, PhysicsMovable},
    render::{tilemap::TileSeq, SpriteResource},
};

use super::{ai::AiMover, CreatureStats, DamageEvent, DamageType, Team};

#[derive(Copy, Clone)]
pub enum ProjectileType {
    RedSpikes,
    BlueBlob,
    Shock,
    RockLarge,
    RockSmall,
    Fire,
}

impl ProjectileType {
    pub fn damage(&self) -> (i16, DamageType) {
        type DT = DamageType;
        match self {
            ProjectileType::BlueBlob => (15, DT::Normal),
            ProjectileType::RedSpikes => (10, DT::Normal),
            ProjectileType::RockLarge => (12, DT::Normal),
            ProjectileType::RockSmall => (3, DT::Normal),
            ProjectileType::Fire => (7, DT::Normal),
            ProjectileType::Shock => (20, DT::Normal),
        }
    }

    pub fn speed(&self) -> f32 {
        match self {
            ProjectileType::BlueBlob => 8.0,
            ProjectileType::RedSpikes => 6.0,
            ProjectileType::RockLarge => 6.0,
            ProjectileType::RockSmall => 6.0,
            ProjectileType::Fire => 6.0,
            ProjectileType::Shock => 2.0,
        }
    }

    pub fn make_uv(&self) -> TileSeq {
        match self {}
    }

    fn make_projectile(&self, team: Team) -> Projectile {
        let (damage, dam_type) = self.damage();
        Projectile {
            team,
            damage,
            dam_type,
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub team: Team,
    pub damage: i16,
    pub dam_type: DamageType,
}

pub fn spawn_projectile(
    ptype: ProjectileType,
    team: Team,
    pos: Vec3,
    dir: Vec3,
    max_dist: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    render_res: &mut ResMut<SpriteResource>,
) {
    let velocity = dir * ptype.speed();

    let uv = ptype.make_uv();

    let mut proto_projectile = commands.spawn(uv.to_sprite_bundle(pos, meshes, render_res));
    proto_projectile.insert(crate::render::Animation::new(uv, 0.1));
    proto_projectile.insert(ptype.make_projectile(team));
    proto_projectile.insert(PhysicsBody::new(0.10, MapCollisionEvent::Destroy)); // TODO: Electricity should have a higher radius.
    proto_projectile.insert(PhysicsMovable::new(velocity, false));

    if max_dist.is_finite() {
        proto_projectile.insert(crate::lifecycle::Ttl::new(max_dist / ptype.speed()));
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
                instigator: None, // TODO: Implement
                target: target_entity,
                damage: projectile.damage,
                dam_type: projectile.dam_type,
            });
            // TODO: Send event

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
    for ev in ev_damage.iter() {
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
