use bevy::{prelude::*};

use crate::{map::{MapData, Coords}, combat::CreatureStats, combat::Team, rendering::{TexCoords, SpriteResource}};

#[derive(Copy,Clone)]
pub enum MonsterType {
    Imp,
    EyeMonster,
    Goliath,
}

impl MonsterType {
    pub fn make_ai(&self) -> AI {
        use MonsterType::*;
        match self {
            Imp => {AI::new(2.5)},
            EyeMonster => {AI::new(5.0)},
            Goliath => { AI::new(9.0)}
        }
    }

    pub fn make_stats(&self) -> CreatureStats {
        use MonsterType::*;
        match self {
            Imp => {CreatureStats{
                speed: 6.,
                hp: 1,
                hp_max: 1,
                team: Team::Monsters,
            }},
            EyeMonster => {CreatureStats{
                speed: 6.,
                hp: 2,
                hp_max: 2,
                team: Team::Monsters,
            }},
            Goliath => {CreatureStats{
                speed: 8.,
                hp: 4,
                hp_max: 4,
                team: Team::Monsters,
            }}
        }
    }

    pub fn make_weapon(&self) -> crate::weapon::Weapon {
        use MonsterType::*;
        match self {
            Imp => {crate::weapon::Weapon::new(crate::weapon::ProjectileType::Shock, 1.8)},
            EyeMonster => {crate::weapon::Weapon::new(crate::weapon::ProjectileType::RedSpikes, 0.6)},
            Goliath => {crate::weapon::Weapon::new(crate::weapon::ProjectileType::RedSpikes, 0.9)}
        }
    }

    pub fn rand() -> Self {
        let r = fastrand::u32(1..=6);
        if r < 3 {
            Self::Imp
        } else if r < 6 {
            Self::EyeMonster
        } else {
            Self::Goliath
        }
    }

    fn make_uv(&self) -> TexCoords {
        use MonsterType::*;
        match self {
            Imp => TexCoords::new(0..1, 7),
            EyeMonster => TexCoords::new(4..7, 7),
            Goliath => TexCoords::new(8..9, 7)
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
    sight_radius : f32,
    state : AIState,
}

impl AI {
    fn new(sight_radius : f32) -> Self{
        Self {
            sight_radius,
            state : AIState::PlayerUnknown,
        }
    }
}

pub fn spawn_monster(
    commands: &mut Commands,
    map_data: &ResMut<crate::map::MapData>,
    meshes: &mut ResMut<Assets<Mesh>>,
    render_res : &mut ResMut<SpriteResource>,
) {
    // TODO: LoS check
    // TODO: Monster check
    let pos = map_data.map.random_square();
    let monster_type = MonsterType::rand();
    let uv = monster_type.make_uv();

    commands.spawn(uv.to_sprite_bundle(pos.to_vec(), 0.3, meshes, render_res))
        .insert(crate::rendering::FaceCamera)
        .insert(monster_type.make_ai())
        .insert(monster_type.make_stats())
        .insert(monster_type.make_weapon());
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
    mut monster_query: Query<(&AI, &mut crate::weapon::Weapon)>,
) {
    for (ai, mut weapon) in monster_query.iter_mut() {
        use crate::weapon::FireMode::*;
        let firing = match ai.state {
            AIState::SeePlayer(pos) => FireAt(pos),
            _ => NoFire,
        };
        weapon.set_fire_state(firing);
    }
}