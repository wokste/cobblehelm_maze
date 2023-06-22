mod ai;
mod combat;
mod game;
mod grid;
mod map;
mod modelgen;
mod physics;
mod player;
mod procgen;
mod rendering;
mod ui;
mod weapon;

use bevy::prelude::*;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cobblehem Maze".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_state::<game::GameState>()
        .add_plugin(ui::UIPlugin)
        .add_plugin(game::GamePlugin)
        .add_startup_system(app_setup)

        .run();
}

fn app_setup(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut render_res: ResMut<rendering::SpriteResource>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.5;

    let texture = asset_server.load("sprites/sprites.png");

    render_res.material = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        alpha_mode: AlphaMode::Mask(0.5),
        unlit: true,
        ..default()
        //Color::WHITE.into()
    });
}

// This resource tracks the game's score
#[derive(Resource)]
pub struct GameInfo {
    pub hp_perc: f32,
    pub score: i32,
    pub coins : i32,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            hp_perc: 1.0,
            score: 0,
            coins: 0
        }
    }
}