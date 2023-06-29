use bevy::prelude::*;

use crate::game::GameState;

pub mod ai;
pub mod player;
pub mod weapon;

pub struct CombatPlugin;

impl Plugin for CombatPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(player::InputMap::default())
            .add_systems((
                player::player_input,
                player::update_map,
                ai::ai_los.after(player::update_map),
                ai::ai_fire.after(ai::ai_los),
                weapon::check_projectile_creature_collisions,
                weapon::fire_weapons.after(player::player_input).after(ai::ai_fire)
            ).in_set(OnUpdate(GameState::InGame)));
    }
}

#[derive(Copy,Clone)]
pub enum MonsterType {
    Imp,
    EyeMonster,
    Goliath,
    Laima,
    IronGolem,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Team {
    Players,
    Monsters,
//    Environment,
}

#[derive(Component)]
pub struct CreatureStats {
    pub speed: f32,
    pub hp: i16,
    pub hp_max: i16,
    pub team: Team,
}

impl CreatureStats {
    pub fn player() -> Self {
        Self {
            speed: 6.0,
            hp: 100,
            hp_max: 100,
            team: Team::Players,
        }
    }
}