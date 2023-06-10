use bevy::prelude::*;
use crate::game::GameState;

pub mod hud;
pub mod menus;
pub mod styles;

pub struct UIPlugin;

impl Plugin for UIPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system(menus::spawn_main_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(menus::spawn_gameover_menu.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(menus::spawn_pause_menu.in_schedule(OnEnter(GameState::Paused)))
            .add_system(menus::despawn_menu.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(menus::despawn_menu.in_schedule(OnExit(GameState::GameOver)))
            .add_system(menus::despawn_menu.in_schedule(OnExit(GameState::Paused)))
            .add_system(menus::interact_with_button.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(menus::interact_with_button.in_set(OnUpdate(GameState::GameOver)))
            .add_system(menus::interact_with_button.in_set(OnUpdate(GameState::Paused)))
            
            .add_system(hud::spawn.in_schedule(OnEnter(GameState::InGame)))
            .add_system(hud::despawn.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(hud::despawn.in_schedule(OnEnter(GameState::MainMenu)))

            .add_system(hud::update_hud.in_set(OnUpdate(GameState::InGame)))
            ;
    }
}