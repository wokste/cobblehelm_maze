use bevy::prelude::*;

use crate::{
    game::GameState,
    ui::menus::{MenuInfo, MenuType},
};

use self::ai::AiMover;

pub mod ai;
pub mod player;
pub mod projectile;
pub mod weapon;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(player::InputMap::default())
            .insert_resource(player::InputState::default())
            .add_event::<DamageEvent>()
            .add_event::<player::InputAction>()
            .add_systems(
                Update,
                (
                    player::gamepad_connections,
                    player::get_player_input,
                    player::handle_player_input.after(player::get_player_input),
                    player::update_map,
                    ai::ai_los.after(player::update_map),
                    ai::ai_move.after(ai::ai_los),
                    projectile::check_collisions,
                    projectile::take_damage_system
                        .after(projectile::check_collisions)
                        .after(weapon::fire_weapons),
                    weapon::fire_weapons
                        .after(player::get_player_input)
                        .after(ai::ai_move),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Copy, Clone)]
pub enum MonsterType {
    Imp = 1,
    EyeMonster1,
    EyeMonster2,
    Goliath,
    Laima,
    IronGolem,
    Demon,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Team {
    Players,
    Monsters,
    //    Environment,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DamageType {
    Normal,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct DamageEvent {
    instigator: Option<Entity>,
    target: Entity,
    damage: i16,
    dam_type: DamageType,
}

#[derive(Component)]
pub struct CreatureStats {
    pub speed: f32,
    pub hp: i16,
    pub hp_max: i16,
    pub team: Team,
    pub monster_type: Option<MonsterType>,
}

impl CreatureStats {
    pub fn player() -> Self {
        Self {
            speed: 6.0,
            hp: 100,
            hp_max: 100,
            team: Team::Players,
            monster_type: None,
        }
    }

    pub fn take_damage(
        &mut self,
        evt: &DamageEvent,
        commands: &mut Commands,
        game: &mut ResMut<crate::GameInfo>,
        game_state: &mut ResMut<NextState<crate::game::GameState>>,
        map_data: &mut ResMut<crate::map::MapData>,
        menu_info: &mut ResMut<MenuInfo>,
        ai_pos: Option<&mut AiMover>,
    ) -> bool {
        if evt.damage <= 0 {
            return false;
        }

        self.hp -= evt.damage;
        if self.team == Team::Players {
            game.update_hp(self);
        }

        if !self.alive() {
            if self.team == Team::Players {
                game_state.set(crate::game::GameState::GameMenu);
                menu_info.set(MenuType::GameOver);
            } else {
                commands.entity(evt.target).despawn();
                if let Some(monster_type) = self.monster_type {
                    game.score += monster_type.get_score();
                }

                if let Some(ai_pos) = ai_pos {
                    ai_pos.remove_from(&mut map_data.monster_map);
                }
            }
        };
        true
    }

    pub fn alive(&self) -> bool {
        self.hp > 0
    }

    pub fn get_hurt_sound(&self, asset_server: &Res<AssetServer>) -> Option<Handle<AudioSource>> {
        let sound_name = if self.team == Team::Players {
            "audio/player_hurt.ogg"
        } else {
            "audio/monster_hurt.ogg"
        };
        Some(asset_server.load(sound_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monster_null_optimization() {
        assert_eq!(
            std::mem::size_of::<Option<MonsterType>>(),
            std::mem::size_of::<MonsterType>()
        );
    }
}
