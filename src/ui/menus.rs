use bevy::{prelude::*, app::AppExit};

use crate::game::GameState;
use super::styles::*;


#[derive(Component)]
pub struct Menu;


#[derive(Component)]
pub enum ButtonAction{
    Play,
    Resume,
    ToMainMenu,
    NextLevel,
    Quit,
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>)
{
	make_menu(&mut commands, &asset_server, GameState::MainMenu)
}

pub fn spawn_gameover_menu(mut commands: Commands, asset_server: Res<AssetServer>)
{
	make_menu(&mut commands, &asset_server, GameState::GameOver)
}

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>)
{
	make_menu(&mut commands, &asset_server, GameState::Paused)
}

pub fn spawn_next_level_screen(mut commands: Commands, asset_server: Res<AssetServer>)
{
	make_menu(&mut commands, &asset_server, GameState::NextLevel)
}

pub fn make_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, state: GameState)
{
	let _menu = commands.spawn((Menu, NodeBundle{
        style: MENU_STYLE,
		background_color: Color::DARK_GRAY.into(),
		..default()
	})).with_children(|parent| {
        match state {
            GameState::MainMenu => {
                parent.spawn(make_simple_text(asset_server, "Main Menu", FONT_H1, TextAlignment::Center));
                make_button(parent, asset_server, "Play", ButtonAction::Play);
                make_button(parent, asset_server, "Quit", ButtonAction::Quit);
            },
            GameState::InGame => panic!("No menu should call this"),
            GameState::GameOver => {
                parent.spawn(make_simple_text(asset_server, "Game Over", FONT_H1, TextAlignment::Center));
                make_button(parent, asset_server, "Quit Game", ButtonAction::ToMainMenu);
            },
            GameState::Paused => {
                parent.spawn(make_simple_text(asset_server, "Main Menu", FONT_H1, TextAlignment::Center));
                make_button(parent, asset_server, "Resume", ButtonAction::Resume);
                make_button(parent, asset_server, "Quit Game", ButtonAction::ToMainMenu);
            },
            GameState::NextLevel => {
                parent.spawn(make_simple_text(asset_server, "Level Complete", FONT_H1, TextAlignment::Center));
                make_button(parent, asset_server, "Play Next Level", ButtonAction::NextLevel);
            },
        };
    }).id();
}

fn make_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, text: &str, action : ButtonAction){
    parent.spawn((ButtonBundle{
        style: BUTTON_STYLE,
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    }, action))
    .with_children(|parent| {
        parent.spawn(make_simple_text(asset_server, text, FONT_P, TextAlignment::Center));
    });
}

pub fn despawn_menu(mut commands: Commands, query : Query<Entity, With<Menu>>)
{
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn interact_with_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor, &ButtonAction), Changed<Interaction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<crate::GameInfo>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut background_color, action) in &mut button_query {
        *background_color = match interaction {
            Interaction::Clicked => { Color::rgb(0.25, 0.25, 1.0).into() },
            Interaction::Hovered => { Color::rgb(0.2, 0.2, 0.2).into() },
            Interaction::None => { Color::rgb(0.15, 0.15, 0.15).into() },
        };

        if let Interaction::Clicked = interaction {
            match action {
                ButtonAction::Play => {
                    game_state.set(GameState::InGame);
                },
                ButtonAction::Resume => {
                    game_data.level_spawned = false;
                    game_data.level += 1;

                    game_state.set(GameState::InGame);
                },
                ButtonAction::NextLevel => {
                    game_state.set(GameState::InGame);
                },
                ButtonAction::ToMainMenu => {
                    game_state.set(GameState::MainMenu);
                },
                ButtonAction::Quit => {
                    exit.send(AppExit);
                },
            }
        }
    }
}