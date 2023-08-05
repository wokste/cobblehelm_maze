use crate::game::GameState;
use bevy::prelude::*;

pub mod hud;
pub mod menus;
pub mod styles;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<menus::ButtonAction>()
            .insert_resource(menus::MenuInfo::main_menu())
            .add_systems(OnEnter(GameState::MainMenu), menus::spawn_menu)
            .add_systems(OnEnter(GameState::GameMenu), menus::spawn_menu)
            .add_systems(OnExit(GameState::MainMenu), menus::despawn_menu)
            .add_systems(OnExit(GameState::GameMenu), menus::despawn_menu)
            .add_systems(OnEnter(GameState::InGame), hud::spawn)
            .add_systems(OnExit(GameState::InGame), hud::despawn)
            .add_systems(
                Update,
                (
                    menus::button_mouse,
                    menus::button_keys,
                    menus::button_press
                        .after(menus::button_mouse)
                        .after(menus::button_keys),
                )
                    .run_if(not(in_state(GameState::InGame))),
            )
            .add_systems(
                Update,
                (hud::update_hud.run_if(in_state(GameState::InGame)),),
            );
    }
}
