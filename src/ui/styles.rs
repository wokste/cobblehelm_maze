use bevy::prelude::*;

pub const BT_PRESS_COLOR: Color = Color::rgb(0.25, 0.25, 1.0);
pub const BT_HOVER_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const BT_BASIC_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub const MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.margin = UiRect {
        left: Val::Auto,
        right: Val::Auto,
        top: Val::Auto,
        bottom: Val::Auto,
    };
    style.padding = UiRect {
        left: Val::Px(50.0),
        right: Val::Px(50.0),
        top: Val::Px(50.0),
        bottom: Val::Px(50.0),
    };
    style.row_gap = Val::Px(16.0);
    style.column_gap = Val::Px(16.0);
    style
};

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(250.0);
    style.height = Val::Px(100.0);
    style
};

pub const FONT_P: f32 = 30.0;
pub const FONT_H1: f32 = 60.0;

pub fn make_menu_head(asset_server: &Res<AssetServer>, text: &str) -> TextBundle {
    make_text(asset_server, text, FONT_H1, TextAlignment::Center)
}

pub fn make_text(
    asset_server: &Res<AssetServer>,
    text: &str,
    font_size: f32,
    alignment: TextAlignment,
) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                text,
                TextStyle {
                    font: asset_server.load("other/bit_potion.ttf"),
                    font_size,
                    color: Color::WHITE,
                },
            )],
            alignment,
            ..default()
        },
        ..default()
    }
}
