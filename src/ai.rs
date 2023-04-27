use bevy::{prelude::*};

use crate::{map::{MapData, Coords}, combat::CreatureStats, combat::Team};

#[derive(Copy,Clone)]
pub enum MonsterType {
    EyeMonster,
    Goliath,
}

impl MonsterType {
    pub fn make_ai(&self) -> AI {
        use MonsterType::*;
        match self {
            EyeMonster => {AI::new(5.0)},
            Goliath => { AI::new(9.0)}
        }
    }

    pub fn make_stats(&self) -> CreatureStats {
        use MonsterType::*;
        match self {
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
            EyeMonster => {crate::weapon::Weapon::new(crate::weapon::ProjectileType::Fireball)},
            Goliath => {crate::weapon::Weapon::new(crate::weapon::ProjectileType::Fireball)}
        }
    }

    pub fn rand() -> Self {
        let r = fastrand::u32(1..=6);
        if r < 4 {
            Self::EyeMonster
        } else {
            Self::Goliath
        }
    }

    fn to_color(&self) -> Color {
        use MonsterType::*;
        match self {
            EyeMonster => Color::ORANGE,
            Goliath => Color::LIME_GREEN
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
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let monster_pos = map_data.map.random_square(); // TODO: LoS check
    let monster_type = MonsterType::rand();

    commands.spawn(PbrBundle {
        mesh: meshes.add( Mesh::from(shape::Cube{ size: 0.5 })),
        material: materials.add(StandardMaterial {
            base_color: monster_type.to_color(),
            alpha_mode: AlphaMode::Opaque,
            unlit: true,
            ..default()
            //Color::WHITE.into()
        }),
        transform: Transform::from_xyz(monster_pos.x as f32 + 0.5, 0.5, monster_pos.z as f32 + 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(crate::rendering::Sprite)
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
        weapon.firing = match ai.state {
            AIState::SeePlayer(pos) => FireAt(pos),
            _ => NoFire,
        }
    }
}