use bevy::prelude::*;

pub mod hud;
pub mod main_menu;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum UIState {
    #[default]
    MainMenu,
    InGame,
}

pub struct UIPlugin;

impl Plugin for UIPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_state::<UIState>()
            .add_system(main_menu::spawn.in_schedule(OnEnter(UIState::MainMenu)))
            .add_system(main_menu::despawn.in_schedule(OnExit(UIState::MainMenu)))
            .add_system(main_menu::interact_with_button.in_set(OnUpdate(UIState::MainMenu)))
            
            .add_system(hud::spawn.in_schedule(OnEnter(UIState::InGame)))
            .add_system(hud::despawn.in_schedule(OnExit(UIState::InGame)))
            ;
    }
}