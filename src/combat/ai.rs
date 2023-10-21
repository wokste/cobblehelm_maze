use bevy::prelude::*;
use bitflags::bitflags;
use tinyvec::ArrayVec;

use super::{weapon::*, *};
use crate::{
    combat::projectile::ProjectileType,
    grid::{Coords, Grid},
    map::MapData,
    physics::MapCollisionEvent,
    render::{spritemap::SpriteSeq, RenderResource},
};

type RealF32 = ordered_float::NotNan<f32>;

fn rf32(f: f32) -> RealF32 {
    RealF32::new(f).expect("Don't use this function on anything that could be NAN")
}

const SIGHT_RADIUS: f32 = 16.0;

impl MonsterType {
    pub fn make_ai(&self) -> AI {
        use MonsterType as MT;
        match self {
            MT::Imp => AI::new(Flags::Approach | Flags::Follow),
            MT::Goblin => AI::new(Flags::Approach | Flags::Follow),
            MT::EyeMonster1 => AI::new(Flags::Follow),
            MT::Ettin => AI::new(Flags::Approach),
            MT::Laima => AI::new(Flags::Approach),
            MT::IronGolem => AI::new(Flags::empty()),
            MT::EyeMonster2 => AI::new(Flags::empty()),
            MT::Demon => AI::new(Flags::Approach | Flags::Follow),
        }
    }

    pub fn jumps(&self) -> bool {
        use MonsterType as MT;
        matches!(
            self,
            MT::Imp | MT::Goblin | MT::Ettin | MT::IronGolem | MT::Demon
        )
    }

    pub fn make_stats(&self) -> CreatureStats {
        use MonsterType as MT;
        let (speed, hp) = match self {
            MT::Imp => (2.5, 5),
            MT::Goblin => (3.0, 5),
            MT::EyeMonster1 => (0.0, 10),
            MT::EyeMonster2 => (2.0, 10),
            MT::Ettin => (2.0, 20),
            MT::Laima => (1.5, 18),
            MT::IronGolem => (1.0, 40),
            MT::Demon => (1.0, 20),
        };
        CreatureStats {
            speed,
            hp,
            hp_max: hp,
            team: Team::Monsters,
            monster_type: Some(*self),
        }
    }

    pub fn make_weapon(&self) -> Weapon {
        use MonsterType as MT;
        match self {
            MT::Imp => Weapon::new_melee(0.5, 3, DamageType::Normal),
            MT::Goblin => Weapon::new_melee(0.5, 3, DamageType::Normal),
            MT::EyeMonster1 => Weapon::new(
                0.9,
                WeaponEffect::Ranged {
                    ptype: ProjectileType::RedSpikes,
                    max_dist: f32::INFINITY,
                    accuracy: 0.1,
                },
            ),
            MT::EyeMonster2 => Weapon::new_ranged(0.6, ProjectileType::RedSpikes, f32::INFINITY),
            MT::Ettin => Weapon::new(
                0.9,
                WeaponEffect::Ranged {
                    ptype: ProjectileType::Rock,
                    max_dist: 9.0,
                    accuracy: 0.1,
                },
            ),
            MT::Laima => Weapon::new_ranged(1.2, ProjectileType::Shock, 4.0),
            MT::IronGolem => Weapon::new_ranged(0.7, ProjectileType::RedSpikes, f32::INFINITY),
            MT::Demon => Weapon::new(
                0.9,
                WeaponEffect::RangedArc {
                    ptype: ProjectileType::Fire,
                    max_dist: f32::INFINITY,
                    arc: 0.6,
                    count: 5,
                },
            ),
        }
    }

    fn get_tile_seq(&self, tiles: &crate::render::spritemap::SpriteMap) -> SpriteSeq {
        use MonsterType as MT;
        let str = match self {
            MT::Imp => "imp.png",
            MT::Goblin => "goblin.png",
            MT::EyeMonster1 => "eye_monster.png",
            MT::EyeMonster2 => "eye_monster2.png",
            MT::Ettin => "ettin.png",
            MT::Laima => "laima.png",
            MT::IronGolem => "iron_golem.png",
            MT::Demon => "demon_fire.png",
        };
        tiles.get_monster(str)
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        map_data: &mut ResMut<crate::map::MapData>,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<RenderResource>,
        rng: &mut fastrand::Rng,
    ) -> Result<(), &'static str> {
        let pos = choose_spawn_pos(map_data, rng)?;
        let uv = self.get_tile_seq(&render_res.sprites);

        commands
            .spawn(uv.to_sprite_bundle(pos.to_vec(self.jumps(), 0.0), meshes, render_res))
            .insert(crate::render::Animation::new(uv, rng.f32() * 0.04 + 0.16))
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

    pub fn get_score(&self) -> i32 {
        match self {
            MonsterType::Imp => 20,
            MonsterType::Goblin => 20,
            MonsterType::EyeMonster1 => 50,
            MonsterType::EyeMonster2 => 70,
            MonsterType::Ettin => 100,
            MonsterType::Laima => 30,
            MonsterType::IronGolem => 140,
            MonsterType::Demon => 170,
        }
    }
}

pub enum AIState {
    PlayerUnknown,
    SeePlayer(Vec3),
    SawPlayer(Coords),
}
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Flags: u32 {
        const None = 0;
        const Approach = 0x1;
        const Follow = 0x2;
    }
}

#[derive(Component)]
pub struct AI {
    state: AIState,
    flags: Flags,
}

impl AI {
    pub fn get_fire_dir(&self, self_pos: &Vec3) -> Option<Vec3> {
        match self.state {
            AIState::SeePlayer(pos) => Some((pos - *self_pos).normalize()),
            _ => None,
        }
    }

    fn new(flags: Flags) -> Self {
        Self {
            state: AIState::PlayerUnknown,
            flags,
        }
    }
}

#[derive(Component)]
pub struct AiMover {
    from: Coords,
    to: Coords,
    f: f32,
}

impl AiMover {
    fn new(pos: Coords, has_monster_grid: &mut Grid<bool>) -> Self {
        debug_assert!(!has_monster_grid[pos]);
        has_monster_grid[pos] = true;

        Self {
            from: pos,
            to: pos,
            f: 1.0,
        }
    }

    pub fn is_removed(&self) -> bool {
        self.from == Coords::INVALID
    }

    pub fn to_vec(&self, jumps: bool, speed: f32) -> Vec3 {
        let height = if jumps {
            let jump_count = (3.0 / speed).ceil();
            let jump_time = 1.0 / jump_count / speed;
            let jump_height = jump_time * jump_time * 1.5;
            let f = (self.f * jump_count).fract();

            0.5 + 4.0 * (f - f * f) * jump_height
        } else {
            0.5
        };

        let from = self.from.to_vec(height);
        let to = self.to.to_vec(height);
        Vec3::lerp(from, to, self.f)
    }

    pub fn set_next_square(&mut self, pos: Coords, has_monster_grid: &mut Grid<bool>) {
        debug_assert!(!self.is_removed());

        let old_pos = self.to;

        debug_assert!(old_pos != pos);
        debug_assert!(has_monster_grid[old_pos]);
        debug_assert!(!has_monster_grid[pos]);

        has_monster_grid[old_pos] = false;
        has_monster_grid[pos] = true;

        self.from = old_pos;
        self.to = pos;
        self.f -= 1.0;
    }

    pub fn remove_from(&mut self, has_monster_grid: &mut Grid<bool>) {
        debug_assert!(!self.is_removed());

        let pos = self.to;

        debug_assert!(has_monster_grid[pos]);
        has_monster_grid[pos] = false;

        self.from = Coords::INVALID;
        self.to = Coords::INVALID;
        self.f = f32::NEG_INFINITY; // Never trigger add_dist again
    }

    pub fn add_dist(&mut self, dist: f32) -> bool {
        self.f += dist;
        self.f >= 1.0
    }
}

fn choose_spawn_pos(
    map_data: &mut ResMut<crate::map::MapData>,
    rng: &mut fastrand::Rng,
) -> Result<AiMover, &'static str> {
    for _ in 0..4096 {
        let pos = map_data.solid_map.size().shrink(1).rand(rng);

        if map_data.solid_map[pos] || map_data.monster_map[pos] {
            continue;
        }

        if map_data.can_see_player(pos.to_vec(0.5), 15.0) {
            continue;
        }

        return Ok(AiMover::new(pos, &mut map_data.monster_map));
    }
    Err("Could not find a proper spawn pos")
}

pub fn ai_los(map_data: Res<MapData>, mut monster_query: Query<(&mut AI, &Transform)>) {
    for (mut ai, transform) in monster_query.iter_mut() {
        if map_data.can_see_player(transform.translation, SIGHT_RADIUS) {
            ai.state = AIState::SeePlayer(map_data.player_pos.translation);
        } else if let AIState::SeePlayer(pos) = ai.state {
            if ai.flags.contains(Flags::Follow) {
                ai.state = AIState::SawPlayer(Coords::from_vec(pos))
            } else {
                ai.state = AIState::PlayerUnknown
            }
        }
    }
}

struct FuzzyPath {
    dirs: ArrayVec<[(Coords, RealF32); 8]>,
    src: Coords,
}

impl FuzzyPath {
    fn init(map_data: &MapData, src: Coords) -> Self {
        let zero: RealF32 = RealF32::new(0.0).unwrap();
        let mut dirs = ArrayVec::<[(Coords, RealF32); 8]>::new();

        // Find all directions the creature can move towards
        for dz in -1..=1 {
            for dx in -1..=1 {
                let dir = Coords::new(dx, dz);
                if dir == Coords::ZERO {
                    continue; // No move at all
                }

                let dest = src + dir;
                if map_data.solid_map[dest] || map_data.monster_map[dest] {
                    continue; // Tile is blocked
                }

                if let Some((h, v)) = dir.split() {
                    if map_data.solid_map[src + h] || map_data.solid_map[src + v] {
                        continue; // Tile would require corner cutting
                    }
                }

                // Tile is free
                dirs.push((dir, zero));
            }
        }
        Self { dirs, src }
    }

    fn choose(&self) -> Option<Coords> {
        self.dirs
            .iter()
            .max_by_key(|(_, weight)| weight)
            .map(|(dir, _)| self.src + *dir)
    }

    fn add_random(&mut self, weight: RealF32) {
        for (_, val) in &mut self.dirs {
            *val += weight * rf32(fastrand::f32());
        }
    }

    fn add_approach(&mut self, weight: RealF32, to: Coords, on_end: RealF32) {
        let delta_to = to - self.src;

        for (dir, val) in &mut self.dirs {
            let dot = dir.dot_norm(delta_to);
            let dot = RealF32::new(dot).unwrap_or(on_end);

            *val += weight * dot;
        }
    }
}

pub fn ai_move(
    mut map_data: ResMut<MapData>,
    time: Res<Time>,
    mut monster_query: Query<(&mut AI, &mut AiMover, &CreatureStats, &mut Transform)>,
) {
    let time = time.delta().as_secs_f32();
    for (mut ai_state, mut ai_mover, stats, mut transform) in monster_query.iter_mut() {
        if stats.speed == 0.0 {
            continue;
        }

        if ai_mover.add_dist(stats.speed * time) {
            let ai_pos = ai_mover.to;
            let mut fuzzy_path = FuzzyPath::init(&map_data, ai_pos);

            match ai_state.state {
                AIState::PlayerUnknown => {
                    fuzzy_path.add_random(rf32(1.0));
                }
                AIState::SeePlayer(player_pos) => {
                    if ai_state.flags.contains(Flags::Approach) {
                        fuzzy_path.add_approach(
                            rf32(1.0),
                            Coords::from_vec(player_pos),
                            rf32(f32::NEG_INFINITY),
                        );
                    } else {
                        fuzzy_path.add_random(rf32(1.0));
                    }
                }
                AIState::SawPlayer(last_seen_pos) => {
                    fuzzy_path.add_approach(rf32(1.0), last_seen_pos, rf32(1.0));
                }
            };

            let dest_pos = fuzzy_path.choose();

            if let Some(dest_pos) = dest_pos {
                ai_mover.set_next_square(dest_pos, &mut map_data.monster_map);
            } else {
                ai_mover.f = 1.0;
            }

            // Cleanup
            if let AIState::SawPlayer(pos) = ai_state.state {
                if Some(pos) == dest_pos {
                    ai_state.state = AIState::PlayerUnknown;
                }
            };
        }

        let ai_jumps = stats.monster_type.unwrap().jumps(); // TODO: No unwrap here
        transform.translation = ai_mover.to_vec(ai_jumps, stats.speed);
    }
}
