use bevy::prelude::*;

use super::{weapon::*, *};
use crate::{
    grid::{Coords, Grid},
    map::MapData,
    physics::MapCollisionEvent,
    rendering::{SpriteResource, TexCoords},
};

const SIGHT_RADIUS: f32 = 16.0;

impl MonsterType {
    pub fn make_ai(&self) -> AI {
        use MonsterType as MT;
        match self {
            MT::Imp => AI::new(),
            MT::EyeMonster => AI::new(),
            MT::Goliath => AI::new(),
            MT::Laima => AI::new(),
            MT::IronGolem => AI::new(),
        }
    }

    pub fn make_stats(&self) -> CreatureStats {
        use MonsterType as MT;
        let (speed, hp) = match self {
            MT::Imp => (6.0, 5),
            MT::EyeMonster => (6.0, 10),
            MT::Goliath => (8.0, 20),
            MT::Laima => (6.0, 10),
            MT::IronGolem => (8.0, 25),
        };
        CreatureStats {
            speed,
            hp,
            hp_max: hp,
            team: Team::Monsters,
        }
    }

    pub fn make_weapon(&self) -> Weapon {
        use MonsterType as MT;
        match self {
            MT::Imp => Weapon::new(ProjectileType::Shock, 1.8, 3.0),
            MT::EyeMonster => Weapon::new(ProjectileType::RedSpikes, 0.6, f32::INFINITY),
            MT::Goliath => Weapon::new(ProjectileType::RedSpikes, 0.9, f32::INFINITY),
            MT::Laima => Weapon::new(ProjectileType::Shock, 1.2, 4.0),
            MT::IronGolem => Weapon::new(ProjectileType::RedSpikes, 0.7, f32::INFINITY),
        }
    }

    fn make_uv(&self) -> TexCoords {
        use MonsterType as MT;
        match self {
            MT::Imp => TexCoords::new(0..4, 7),
            MT::EyeMonster => TexCoords::new(4..6, 7),
            MT::Goliath => TexCoords::new(8..10, 7),
            MT::Laima => TexCoords::new(12..15, 7),
            MT::IronGolem => TexCoords::new(16..18, 7),
        }
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        map_data: &mut ResMut<crate::map::MapData>,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<SpriteResource>,
        rng: &mut fastrand::Rng,
    ) -> Result<(), &'static str> {
        let pos = choose_spawn_pos(map_data, rng)?;
        let uv = self.make_uv();

        commands
            .spawn(uv.to_sprite_bundle(pos.to_vec(0.5), meshes, render_res))
            .insert(crate::rendering::Animation::new(
                uv.x,
                rng.f32() * 0.04 + 0.16,
            ))
            .insert(self.make_ai())
            .insert(pos)
            .insert(self.make_stats())
            .insert(self.make_weapon())
            .insert(crate::physics::PhysicsBody::new(
                0.5,
                MapCollisionEvent::Stop,
            ));

        Ok(())
    }
}

pub enum AIState {
    PlayerUnknown,
    SeePlayer(Vec3),
    FollowPlayer(Coords),
}

#[derive(Component)]
pub struct AI {
    state: AIState,
}

impl AI {
    fn new() -> Self {
        Self {
            state: AIState::PlayerUnknown,
        }
    }
}

#[derive(Component)]
pub struct AiPos {
    from: Coords,
    to: Coords,
    f: f32,
}

impl AiPos {
    fn new(pos: Coords, grid: &mut Grid<bool>) -> Self {
        debug_assert!(grid[pos] == false);
        grid[pos] = true;

        Self {
            from: pos,
            to: pos,
            f: 1.0,
        }
    }

    pub fn to_vec(&self, height: f32) -> Vec3 {
        let from = self.from.to_vec(height);
        let to = self.to.to_vec(height);
        Vec3::lerp(from, to, self.f)
    }

    pub fn set_next_square(&mut self, pos: Coords, grid: &mut Grid<bool>) {
        debug_assert!(grid[self.to] == true);
        debug_assert!(grid[pos] == false);
        grid[self.to] = false;
        grid[pos] = true;

        self.from = self.to;
        self.to = pos;
        self.f -= 1.0;
    }

    pub fn remove_from(&self, grid: &mut Grid<bool>) {
        debug_assert!(grid[self.to] == true);
        grid[self.to] = false;
    }
}

fn choose_spawn_pos(
    map_data: &mut ResMut<crate::map::MapData>,
    rng: &mut fastrand::Rng,
) -> Result<AiPos, &'static str> {
    for _ in 0..4096 {
        let pos = map_data.solid_map.size().shrink(1).rand(rng);

        if map_data.solid_map[pos] || map_data.monster_map[pos] {
            continue;
        }

        if map_data.can_see_player(pos.to_vec(0.6), 10.0) {
            continue;
        }

        return Ok(AiPos::new(pos, &mut map_data.monster_map));
    }
    Err("Could not find a proper spawn pos")
}

pub fn ai_los(map_data: Res<MapData>, mut monster_query: Query<(&mut AI, &Transform)>) {
    for (mut ai, transform) in monster_query.iter_mut() {
        if map_data.can_see_player(transform.translation, SIGHT_RADIUS) {
            ai.state = AIState::SeePlayer(map_data.player_pos.translation)
        } else if let AIState::SeePlayer(pos) = ai.state {
            ai.state = AIState::FollowPlayer(Coords::from_vec(pos))
        }
    }
}

pub fn ai_fire(mut monster_query: Query<(&AI, &mut Weapon)>) {
    for (ai, mut weapon) in monster_query.iter_mut() {
        use FireMode::*;
        let firing = match ai.state {
            AIState::SeePlayer(pos) => FireAt(pos),
            _ => NoFire,
        };
        weapon.set_fire_state(firing);
    }
}
