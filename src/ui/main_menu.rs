use bevy::{prelude::*, app::AppExit};

use super::UIState;


#[derive(Component)]
pub struct MainMenu;


#[derive(Component)]
pub enum ButtonAction{
    Play,
    Quit
}

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>)
{
	let _menu = commands.spawn((MainMenu, NodeBundle{
        style: Style{
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto
            },
            padding: UiRect {
                left: Val::Px(50.0),
                right: Val::Px(50.0),
                top: Val::Px(50.0),
                bottom: Val::Px(50.0)
            },
            gap: Size::new(Val::Px(16.0),Val::Px(16.0)),
            ..default()
        },
		background_color: Color::DARK_GRAY.into(),
		..default()
	})).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![ TextSection::new("Main Menu", TextStyle{
                    font:asset_server.load("other/BitPotion.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE
                })],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        });
        make_button(parent, &asset_server, "play", ButtonAction::Play);
        make_button(parent, &asset_server, "quit", ButtonAction::Quit);
    }).id();

    // TODO: Spawn buttons
}

fn make_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, text: &'static str, action : ButtonAction){
    parent.spawn((ButtonBundle{
        style: Style{
            justify_content: JustifyContent::Center,
            align_items : AlignItems::Center,
            size: Size::new(Val::Px(250.0), Val::Px(100.0)),
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    }, action))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text {
                sections: vec![ TextSection::new(text, TextStyle{
                    font:asset_server.load("other/BitPotion.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE
                })],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        });
    });
}

pub fn despawn(mut commands: Commands, query : Query<Entity, With<MainMenu>>)
{
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn interact_with_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor, &ButtonAction), Changed<Interaction>>,
    mut game_state: ResMut<NextState<UIState>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, mut background_color, action) in &mut button_query {
        *background_color = match interaction {
            Interaction::Clicked => { Color::rgb(0.25, 0.25, 1.0).into() },
            Interaction::Hovered => { Color::rgb(0.2, 0.2, 0.2).into() },
            Interaction::None => { Color::rgb(0.15, 0.15, 0.15).into() },
        };

        if let Interaction::Clicked = interaction {
            match action {
                ButtonAction::Play => game_state.set(UIState::InGame),
                ButtonAction::Quit => exit.send(AppExit),
            }
        }
    }
}