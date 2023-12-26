use bevy::prelude::*;

use crate::combat::{player::InputState, CreatureStats};

use super::styles::*;

#[derive(Component, PartialEq, Clone, Copy)]
pub enum HudField {
    Hp(i16, i16),
    Score(i32),
    Coins(i32),
    Level(u8),
    Time(f32),
    Dir(f32),
    //    Status,
}

impl HudField {
    fn update_impl(
        &mut self,
        game: &crate::GameInfo,
        stats_query: &Query<&CreatureStats>,
        input: &InputState,
    ) {
        let Some(player) = game.player else {return;};
        match self {
            HudField::Hp(val, max) => {
                let Ok(stats) = stats_query.get(player) else {return;};
                *val = stats.hp;
                *max = stats.hp_max;
            }
            HudField::Score(val) => *val = game.score,
            HudField::Coins(val) => *val = game.coins,
            HudField::Level(val) => *val = game.level,
            HudField::Time(val) => *val = game.time.elapsed_secs(),
            HudField::Dir(val) => *val = input.yaw,
        };
    }

    fn update(
        &mut self,
        game: &crate::GameInfo,
        stats_query: &Query<&CreatureStats>,
        input: &InputState,
    ) -> bool {
        let old = *self;
        self.update_impl(game, stats_query, input);

        *self == old
    }

    fn make_text(&self) -> String {
        match self {
            HudField::Hp(val, max) => format!("HP: {}/{}", val, max),
            HudField::Score(val) => format!("Score: {}", val),
            HudField::Coins(val) => format!("Coins: {}", val),
            HudField::Level(val) => format!("Level: {}", val),
            HudField::Time(val) => format!("Time: {}s", val),
            HudField::Dir(val) => {
                let pi = std::f32::consts::PI;
                let index = (val * 8.0 / pi).round() as usize;
                let str_val = [
                    "N", "NNW", "NW", "WNW", "W", "WSW", "SW", "SSW", "S", "SSE", "SE", "ESE", "E",
                    "ENE", "NE", "NNE",
                ][index % 16];
                format!("Dir: {}", str_val)
            }
        }
    }
}

#[derive(Component)]
pub struct Hud;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _hud = commands
        .spawn((
            Hud,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    row_gap: Val::Px(16.0),
                    column_gap: Val::Px(16.0),

                    ..default()
                },
                background_color: Color::MIDNIGHT_BLUE.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                make_text(&asset_server, "", FONT_P, TextAlignment::Center),
                HudField::Hp(-1, -1),
            ));
            parent.spawn((
                make_text(&asset_server, "", FONT_P, TextAlignment::Center),
                HudField::Score(-1),
            ));
            parent.spawn((
                make_text(&asset_server, "", FONT_P, TextAlignment::Center),
                HudField::Coins(-1),
            ));
            parent.spawn((
                make_text(&asset_server, "", FONT_P, TextAlignment::Center),
                HudField::Level(u8::MAX),
            ));
            parent.spawn((
                make_text(&asset_server, "", FONT_P, TextAlignment::Center),
                HudField::Time(f32::NAN),
            ));
            parent.spawn((
                make_text(&asset_server, "", FONT_P, TextAlignment::Center),
                HudField::Dir(0.0),
            ));
        })
        .id();
}

pub fn update_hud(
    mut query: Query<(&mut Text, &mut HudField)>,
    mut game: ResMut<crate::GameInfo>,
    stats_query: Query<&CreatureStats>,
    input: Res<InputState>,
    time: Res<Time>,
) {
    game.time.tick(time.delta());

    for (mut text, mut updated) in &mut query {
        if !updated.update(&game, &stats_query, &input) {
            continue;
        }

        text.sections[0].value = updated.make_text();
    }
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<Hud>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
