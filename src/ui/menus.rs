use bevy::{app::AppExit, prelude::*};

use super::styles::*;
use crate::{combat::CreatureStats, game::GameState, mapgen::style::LevelIndex, GameSettings};

#[derive(Component)]
pub struct MenuMarker;

#[derive(Resource, Default)]
pub struct MenuInfo {
    selected: Option<Entity>,
    menu_type: Option<MenuType>,
}

impl MenuInfo {
    pub fn main_menu() -> Self {
        Self {
            selected: None,
            menu_type: Some(MenuType::MainMenu),
        }
    }

    pub fn set(&mut self, menu_type: MenuType) {
        self.menu_type = Some(menu_type);
        self.selected = None;
    }

    pub fn unset(&mut self) {
        self.menu_type = None;
        self.selected = None;
    }
}

#[derive(Copy, Clone)]
pub enum MenuType {
    MainMenu,
    GameOver,
    Paused,
    NextLevel(LevelIndex),
    Victory,
}

#[derive(Component)]
pub struct Button {
    action: OnClick,
}

#[derive(Event, Clone, Copy)]
pub enum OnClick {
    Play,
    PlayDaily,
    Resume,
    ToMainMenu,
    NextLevel(LevelIndex),
    BuyHealth,
    Quit,
}

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>, menu: Res<MenuInfo>) {
    make_menu(&mut commands, &asset_server, &menu)
}

pub fn make_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, menu: &MenuInfo) {
    let Some(menu_type) = menu.menu_type
    else {
        assert!(false, "Menu loaded while no menu is configured");
        return;
    };
    let _menu = commands
        .spawn((
            MenuMarker,
            NodeBundle {
                style: MENU_STYLE,
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            match menu_type {
                MenuType::MainMenu => {
                    parent.spawn(make_menu_head(asset_server, "Main Menu"));
                    make_button(parent, asset_server, "Play", OnClick::Play);
                    make_button(parent, asset_server, "Daily Run", OnClick::PlayDaily);
                    make_button(parent, asset_server, "Quit", OnClick::Quit);
                }
                MenuType::GameOver => {
                    parent.spawn(make_menu_head(asset_server, "Game Over"));
                    make_button(parent, asset_server, "Quit Game", OnClick::ToMainMenu);
                }
                MenuType::Paused => {
                    parent.spawn(make_menu_head(asset_server, "Paused"));
                    make_button(parent, asset_server, "Resume", OnClick::Resume);
                    make_button(
                        parent,
                        asset_server,
                        "Buy Health Upgrade",
                        OnClick::BuyHealth,
                    );
                    make_button(parent, asset_server, "Quit Game", OnClick::ToMainMenu);
                }
                MenuType::NextLevel(index) => {
                    parent.spawn(make_menu_head(asset_server, "Level Complete"));
                    make_button(
                        parent,
                        asset_server,
                        "Play Next Level",
                        OnClick::NextLevel(index),
                    );
                }
                MenuType::Victory => {
                    parent.spawn(make_menu_head(asset_server, "You win"));
                    make_button(parent, asset_server, "Continue playing", OnClick::Resume);
                    make_button(parent, asset_server, "Quit", OnClick::ToMainMenu);
                }
            };
        })
        .id();
}

fn make_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    action: OnClick,
) {
    parent
        .spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            Button { action },
        ))
        .with_children(|parent| {
            parent.spawn(make_text(asset_server, text, FONT_P, TextAlignment::Center));
        });
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuMarker>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_mouse(
    mut mouse_query: Query<(Entity, &Interaction), (With<Button>, Changed<Interaction>)>,
    mut button_query: Query<(&mut BackgroundColor, &Button)>,
    mut menu: ResMut<MenuInfo>,
    mut events: EventWriter<OnClick>,
) {
    let mut last_button = None;

    for (entity, interaction) in &mut mouse_query {
        if let Ok((mut background_color, button)) = button_query.get_mut(entity) {
            *background_color = match interaction {
                Interaction::Pressed => BT_PRESS_COLOR.into(),
                Interaction::Hovered => BT_HOVER_COLOR.into(),
                Interaction::None => BT_BASIC_COLOR.into(),
            };

            if let Interaction::Hovered = interaction {
                if menu.selected != Some(entity) {
                    last_button = menu.selected;
                    menu.selected = Some(entity);
                }
            }

            if let Interaction::Pressed = interaction {
                events.send(button.action)
            }
        }
    }

    if let Some(last_button) = last_button {
        if let Ok((mut background_color, _)) = button_query.get_mut(last_button) {
            *background_color = BT_BASIC_COLOR.into();
        }
    }
}

fn cycle_buttons(
    forward: bool,
    menu: &mut ResMut<MenuInfo>,
    menu_query: &Query<&Children, With<MenuMarker>>,
    button_query: &mut Query<(&mut BackgroundColor, &Button)>,
) {
    let children = menu_query.get_single().unwrap(); // TODO: No unwrap
    let children: Vec<_> = children.iter().skip(1).copied().collect(); // TODO: Only cycle through buttons instead of this quirk
    let len = children.len();

    let index = children.iter().position(|e| Some(*e) == menu.selected);
    let new_index: usize = match (forward, index) {
        (true, None) => 0,
        (true, Some(index)) => (index + 1) % len,
        (false, None) => len - 1,
        (false, Some(index)) => (index + len - 1) % len,
    };

    if let Some(old_entity) = menu.selected {
        let (mut bg_color, _) = button_query.get_mut(old_entity).unwrap();
        *bg_color = BT_BASIC_COLOR.into();
    }

    let new_entity = children[new_index];
    menu.selected = Some(new_entity);
    let (mut bg_color, _) = button_query.get_mut(new_entity).unwrap();
    *bg_color = BT_HOVER_COLOR.into();
}

pub fn button_keys(
    keys: Res<Input<KeyCode>>,
    state: Res<crate::combat::player::InputState>,
    pad_buttons: Res<Input<GamepadButton>>,
    mut out_events: EventWriter<OnClick>,
    mut menu: ResMut<MenuInfo>,
    menu_query: Query<&Children, With<MenuMarker>>,
    mut button_query: Query<(&mut BackgroundColor, &Button)>,
) {
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::Down => {
                cycle_buttons(true, &mut menu, &menu_query, &mut button_query);
            }
            KeyCode::Up => {
                cycle_buttons(false, &mut menu, &menu_query, &mut button_query);
            }
            KeyCode::Space | KeyCode::Return => {
                if let Some(entity) = menu.selected {
                    let (_, button) = button_query.get(entity).unwrap();
                    out_events.send(button.action);
                }
            }
            // TODO: If the button press to enter menu is changed into just_pressed, this would work
            //KeyCode::Escape => {
            //    out_events.send(OnClick::Resume);
            //}
            _ => {}
        }
    }

    if let Some(gamepad) = state.gamepad {
        let up_button = GamepadButton::new(gamepad, GamepadButtonType::DPadUp);
        let down_button = GamepadButton::new(gamepad, GamepadButtonType::DPadDown);
        let a_button = GamepadButton::new(gamepad, GamepadButtonType::South);
        let b_button = GamepadButton::new(gamepad, GamepadButtonType::East);

        if pad_buttons.just_pressed(up_button) {
            cycle_buttons(true, &mut menu, &menu_query, &mut button_query);
        }
        if pad_buttons.just_pressed(down_button) {
            cycle_buttons(false, &mut menu, &menu_query, &mut button_query);
        }
        if pad_buttons.just_pressed(a_button) {
            if let Some(entity) = menu.selected {
                let (_, button) = button_query.get(entity).unwrap();
                out_events.send(button.action);
            }
        }
        if pad_buttons.just_pressed(b_button) {
            out_events.send(OnClick::Resume);
        }
    };
}

pub fn button_press(
    mut events: EventReader<OnClick>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<crate::GameInfo>,
    mut exit: EventWriter<AppExit>,
    mut game_settings: ResMut<GameSettings>,
    cl_args: Res<crate::CommandLineArgs>,
    mut menu_info: ResMut<MenuInfo>,
    mut stats_query: Query<&mut CreatureStats>,
) {
    for action in events.read() {
        match action {
            OnClick::Play => {
                *game_settings = GameSettings::from_cl(&cl_args);
                game_state.set(GameState::InGame);
                menu_info.unset();
            }
            OnClick::PlayDaily => {
                *game_settings = GameSettings::from_daily(std::time::SystemTime::now());
                game_state.set(GameState::InGame);
                menu_info.unset();
            }
            OnClick::Resume => {
                game_state.set(GameState::InGame);
                menu_info.unset();
            }
            OnClick::NextLevel(level_index) => {
                game_data.next_level(*level_index);
                game_state.set(GameState::InGame);
                menu_info.unset();
            }
            OnClick::ToMainMenu => {
                *game_data = Default::default();
                game_state.set(GameState::MainMenu);
                menu_info.set(MenuType::MainMenu);
            }
            OnClick::Quit => {
                exit.send(AppExit);
            }
            OnClick::BuyHealth => {
                // TODO: No unwrap
                // TODO: Store the level somewhere else
                // TODO: More upgradable stuff
                if let Ok(mut stats) = stats_query.get_mut(game_data.player.unwrap()) {
                    let level = (stats.hp_max / 20 - 2) as i32;
                    let price = 5 * level * (level + 1) / 2;
                    if game_data.coins < price {
                        return;
                    }

                    game_data.coins -= price;

                    stats.hp += 20;
                    stats.hp_max += 20;
                }
            }
        }
    }
}
