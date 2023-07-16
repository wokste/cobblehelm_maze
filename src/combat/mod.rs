use bevy::prelude::*;

use crate::game::GameState;

use self::ai::AiMover;

pub mod ai;
pub mod player;
pub mod weapon;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(player::InputMap::default())
            .insert_resource(player::InputState::default())
            .add_systems(
                (
                    player::player_input,
                    player::update_map,
                    ai::ai_los.after(player::update_map),
                    ai::ai_fire.after(ai::ai_los),
                    ai::ai_move.after(ai::ai_fire),
                    weapon::check_projectile_creature_collisions,
                    weapon::fire_weapons
                        .after(player::player_input)
                        .after(ai::ai_fire),
                )
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}

#[derive(Copy, Clone)]
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

pub struct Damage {
    value: i16,
}

impl Damage {
    pub fn new(value: i16) -> Self {
        Self { value }
    }
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

    pub fn take_damage(
        &mut self,
        entity: Entity,
        damage: Damage,
        commands: &mut Commands,
        game: &mut ResMut<crate::GameInfo>,
        game_state: &mut ResMut<NextState<crate::game::GameState>>,
        map_data: &mut ResMut<crate::map::MapData>,
        ai_pos: Option<&AiMover>,
    ) -> bool {
        self.hp -= damage.value;
        if self.team == Team::Players {
            game.update_hp(&self);
        }

        if self.hp <= 0 {
            if self.team == Team::Players {
                game_state.set(crate::game::GameState::GameOver);
            } else {
                commands.entity(entity).despawn();
                game.score += 50; // TODO: What kind of score to use?
                if let Some(ai_pos) = ai_pos {
                    ai_pos.remove_from(&mut map_data.monster_map);
                }
            }
        }
        self.hp <= 0
    }
}
