use crate::game::GameState;
use bevy::prelude::*;

pub mod hud;
pub mod menus;
pub mod styles;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::MainMenu), menus::spawn_main_menu)
            .add_systems(OnEnter(GameState::GameOver), menus::spawn_gameover_menu)
            .add_systems(OnEnter(GameState::Paused), menus::spawn_pause_menu)
            .add_systems(OnEnter(GameState::NextLevel), menus::spawn_next_level_menu)
            .add_systems(OnExit(GameState::MainMenu), menus::despawn_menu)
            .add_systems(OnExit(GameState::GameOver), menus::despawn_menu)
            .add_systems(OnExit(GameState::Paused), menus::despawn_menu)
            .add_systems(OnExit(GameState::NextLevel), menus::despawn_menu)
            .add_systems(OnEnter(GameState::InGame), hud::spawn)
            .add_systems(OnExit(GameState::InGame), hud::despawn)
            .add_systems(
                Update,
                (
                    menus::interact_with_button.run_if(not(in_state(GameState::InGame))),
                    hud::update_hud.run_if(in_state(GameState::InGame)),
                ),
            );
    }
}
