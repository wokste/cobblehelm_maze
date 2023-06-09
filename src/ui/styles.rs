use bevy::prelude::*;

pub const MENU_STYLE : Style = Style{
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
    ..Style::DEFAULT
};

pub const BUTTON_STYLE : Style = Style{
    justify_content: JustifyContent::Center,
    align_items : AlignItems::Center,
    size: Size::new(Val::Px(250.0), Val::Px(100.0)),
    ..Style::DEFAULT
};

pub const FONT_P : f32 = 50.0;
pub const FONT_H1 : f32 = 50.0;

pub fn make_simple_text(asset_server: &Res<AssetServer>, text : &str, font_size: f32, alignment: TextAlignment) -> TextBundle{
    TextBundle {
        text: Text {
            sections: vec![ TextSection::new(text, TextStyle{
                font:asset_server.load("other/BitPotion.ttf"),
                font_size,
                color: Color::WHITE
            })],
            alignment,
            ..default()
        },
        ..default()
    }

}