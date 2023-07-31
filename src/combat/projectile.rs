use bevy::prelude::*;

use crate::{
    physics::{MapCollisionEvent, PhysicsBody, PhysicsMovable},
    rendering::{SpriteResource, TexCoords},
};

use super::{ai::AiMover, CreatureStats, Team};

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
    pub fn damage(&self) -> i16 {
        match self {
            ProjectileType::BlueBlob => 15,
            ProjectileType::RedSpikes => 10,
            ProjectileType::RockLarge => 12,
            ProjectileType::RockSmall => 3,
            ProjectileType::Fire => 7,
            ProjectileType::Shock => 20,
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

    pub fn make_uv(&self) -> TexCoords {
        match self {
            ProjectileType::RedSpikes => TexCoords::half(0..1, 12),
            ProjectileType::BlueBlob => TexCoords::half(1..2, 12),
            ProjectileType::Shock => TexCoords::basic(2..5, 6),
            ProjectileType::RockLarge => TexCoords::half(0..1, 13),
            ProjectileType::RockSmall => TexCoords::half(1..2, 13),
            ProjectileType::Fire => TexCoords::half(2..4, 12),
        }
    }

    fn make_projectile(&self, team: Team) -> Projectile {
        Projectile {
            team,
            damage: self.damage(),
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub team: Team,
    pub damage: i16,
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
    proto_projectile.insert(crate::rendering::Animation::new(uv.x_range(), 0.1));
    proto_projectile.insert(ptype.make_projectile(team));
    proto_projectile.insert(PhysicsBody::new(0.10, MapCollisionEvent::Destroy)); // TODO: Electricity should have a higher radius.
    proto_projectile.insert(PhysicsMovable::new(velocity, false));

    if max_dist.is_finite() {
        proto_projectile.insert(crate::lifecycle::Ttl::new(max_dist / ptype.speed()));
    }
}

pub fn check_projectile_creature_collisions(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &PhysicsBody, &Transform)>,
    mut target_query: Query<(
        Entity,
        &PhysicsBody,
        &mut CreatureStats,
        &Transform,
        Option<&mut AiMover>,
    )>,
    mut game: ResMut<crate::GameInfo>,
    mut game_state: ResMut<NextState<crate::game::GameState>>,
    mut map_data: ResMut<crate::map::MapData>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (projectile_entity, projectile, projectile_body, projectile_transform) in
        projectile_query.iter_mut()
    {
        for (target_entity, target_body, mut stats, target_transform, mut ai_pos) in
            target_query.iter_mut()
        {
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

            let hurt = stats.take_damage(
                target_entity,
                super::Damage::new(projectile.damage),
                &mut commands,
                &mut game,
                &mut game_state,
                &mut map_data,
                ai_pos.as_deref_mut(),
            );

            if hurt {
                if let Some(sound) = stats.get_hurt_sound(&asset_server) {
                    audio.play(sound);
                }
            }

            commands.entity(projectile_entity).despawn();
        }
    }
}
