use bevy::prelude::*;

use super::styles::*;

#[derive(Component)]
pub struct HudUpdated {
    value: i32,
    field: HudField,
}


enum HudField{
    HpPerc,
    Score,
    Coins,
    Level,
    Time,
//    Status,
}

impl HudUpdated {
    fn update(&mut self, game: &crate::GameInfo) -> bool {
        let new_value: i32 = match self.field {
            HudField::HpPerc => (game.hp_perc * 100.0) as i32,
            HudField::Score => game.score,
            HudField::Coins => game.coins,
            HudField::Level => game.level as i32,
            HudField::Time => self.value + 1, // TODO: Actually measure time
        };

        if self.value == new_value {
            false
        } else {
            self.value = new_value;
            true
        }
    }

    fn make_text(&self) -> String {
        match self.field {
            HudField::HpPerc => format!("HP: {}%", self.value),
            HudField::Score => format!("Score: {}", self.value),
            HudField::Coins => format!("Coins: {}", self.value),
            HudField::Level => format!("Level: {}", self.value),
            HudField::Time => format!("Time: {}", self.value),
        }
    }
}

#[derive(Component)]
pub struct Hud;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
)
{
	let _hud = commands.spawn((Hud, NodeBundle{
        style: Style{
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
            gap: Size::new(Val::Px(16.0),Val::Px(16.0)),
            
            ..default()
        },
		background_color: Color::MIDNIGHT_BLUE.into(),
		..default()
	})).with_children(|parent| {
        parent.spawn((make_simple_text(&asset_server, "", FONT_P, TextAlignment::Center), HudUpdated{field: HudField::HpPerc, value: -1} ) );
        parent.spawn((make_simple_text(&asset_server, "", FONT_P, TextAlignment::Center), HudUpdated{field: HudField::Score, value: -1} ) );
        parent.spawn((make_simple_text(&asset_server, "", FONT_P, TextAlignment::Center), HudUpdated{field: HudField::Coins, value: -1} ) );
        parent.spawn((make_simple_text(&asset_server, "", FONT_P, TextAlignment::Center), HudUpdated{field: HudField::Level, value: -1} ) );
        parent.spawn((make_simple_text(&asset_server, "", FONT_P, TextAlignment::Center), HudUpdated{field: HudField::Time, value: -1} ) );
    })
    .id();
}

pub fn update_hud(
    mut query: Query<(&mut Text, &mut HudUpdated)>,
    game: Res<crate::GameInfo>,
)
{
    for (mut text, mut updated) in &mut query {
        if !updated.update(&game) {
            continue;
        }

        text.sections[0].value = updated.make_text();
    }
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<Hud>>)
{
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}