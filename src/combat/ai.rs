use bevy::{prelude::*};

use crate::{grid::{Coords}, map::{MapData}, rendering::{TexCoords, SpriteResource}, physics::MapCollisionEvent};
use super::{*, weapon::*};

impl MonsterType {
    pub fn make_ai(&self) -> AI {
        use MonsterType::*;
        match self {
            Imp => {AI::new(2.5)},
            EyeMonster => {AI::new(5.0)},
            Goliath => { AI::new(9.0)},
            Laima => {AI::new(5.0)},
            IronGolem => { AI::new(9.0)},
        }
    }

    pub fn make_stats(&self) -> CreatureStats {
        use MonsterType::*;
        let (speed, hp) = match self {
            Imp        => (6.0, 5),
            EyeMonster => (6.0, 10),
            Goliath    => (8.0, 20),
            Laima      => (6.0, 10),
            IronGolem  => (8.0, 25),
        };
        CreatureStats{
            speed,
            hp,
            hp_max: hp,
            team: Team::Monsters,
        }
    }

    pub fn make_weapon(&self) -> Weapon {
        use MonsterType::*;
        match self {
            Imp => {Weapon::new(ProjectileType::Shock, 1.8)},
            EyeMonster => {Weapon::new(ProjectileType::RedSpikes, 0.6)},
            Goliath => {Weapon::new(ProjectileType::RedSpikes, 0.9)}
            Laima => {Weapon::new(ProjectileType::Shock, 1.2)},
            IronGolem => {Weapon::new(ProjectileType::RedSpikes, 0.7)}
        }
    }

    fn make_uv(&self) -> TexCoords {
        use MonsterType::*;
        match self {
            Imp => TexCoords::new(0..4, 7),
            EyeMonster => TexCoords::new(4..6, 7),
            Goliath => TexCoords::new(8..10, 7),
            Laima => TexCoords::new(12..15, 7),
            IronGolem => TexCoords::new(16..18, 7),
        }
    }
}

pub enum AIState {
    PlayerUnknown,
    SeePlayer(Vec3),
    FollowPlayer(Coords),
}

#[derive(Component)]
pub struct AI{
    sight_radius: f32,
    state: AIState,
}

impl AI {
    fn new(sight_radius: f32) -> Self{
        Self {
            sight_radius,
            state: AIState::PlayerUnknown,
        }
    }
}

fn choose_spawn_pos(map_data: &crate::map::MapData, rng: &mut fastrand::Rng) -> Result<Coords, &'static str> {
    
    let map = &map_data.map;//.random_square();
    for _ in 0 .. 4096 {
        let x = rng.i32(1 .. map.x_max() - 1);
        let z = rng.i32(1 .. map.z_max() - 1);

        if map[(x,z)].is_solid() {
            continue;
        }

        let pos = Coords::new(x, z);
        if map_data.can_see_player(pos.to_vec(0.6), 10.0) {
            continue;
        }

        // TODO: Monster check (Multiple monsters at the same spot)

        return Ok(pos);
    }
    Err("Could not find a proper spawn pos")
}

pub fn spawn_monster(
    commands: &mut Commands,
    map_data: &ResMut<crate::map::MapData>,
    monster_type: MonsterType,
    meshes: &mut ResMut<Assets<Mesh>>,
    render_res: &mut ResMut<SpriteResource>,
    rng: &mut fastrand::Rng,
) -> Result<(),&'static str> {
    let pos = choose_spawn_pos(map_data, rng)?;
    let uv = monster_type.make_uv();

    let anim_speed = rng.f32() * 0.04 + 0.16;
    commands.spawn(uv.to_sprite_bundle(pos.to_vec(0.5), anim_speed, meshes, render_res))
        .insert(crate::rendering::FaceCamera)
        .insert(monster_type.make_ai())
        .insert(monster_type.make_stats())
        .insert(monster_type.make_weapon())
        .insert(crate::physics::PhysicsBody::new(0.5, MapCollisionEvent::Stop));
    
    Ok(())
}

pub fn ai_los(
    map_data: Res<MapData>,
    mut monster_query: Query<(&mut AI, &Transform)>,
) {
    for (mut ai, transform) in monster_query.iter_mut() {
        if map_data.can_see_player(transform.translation, ai.sight_radius) {
            ai.state = AIState::SeePlayer(map_data.player_pos)
        } else if let AIState::SeePlayer(pos) = ai.state {
            ai.state = AIState::FollowPlayer(Coords::from_vec(pos))
        }
    }
}

pub fn ai_fire(
    mut monster_query: Query<(&AI, &mut Weapon)>,
) {
    for (ai, mut weapon) in monster_query.iter_mut() {
        use FireMode::*;
        let firing = match ai.state {
            AIState::SeePlayer(pos) => FireAt(pos),
            _ => NoFire,
        };
        weapon.set_fire_state(firing);
    }
}