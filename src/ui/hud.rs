use bevy::prelude::*;

#[derive(Component)]
pub enum HudUpdated{
    HP,
    Score,
    Coins,
//    Status,
}

#[derive(Component)]
pub struct HUD;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>)
{
	let _hud = commands.spawn((HUD, NodeBundle{
        style: Style{
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.0), Val::Px(150.0)),
            gap: Size::new(Val::Px(16.0),Val::Px(16.0)),
            
            ..default()
        },
		background_color: Color::MIDNIGHT_BLUE.into(),
		..default()
	})
    ).id();

    // TODO: Spawn HUD elements
}

pub fn despawn(mut commands: Commands, query : Query<Entity, With<HUD>>)
{
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}