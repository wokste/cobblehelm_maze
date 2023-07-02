mod combat;
mod game;
mod grid;
mod map;
mod modelgen;
mod pickup;
mod physics;
mod mapgen;
mod rendering;
mod ui;

use bevy::{prelude::*, time::Stopwatch};

#[derive(Component)]
pub struct LevelObject;

use clap::Parser;

#[derive(Debug, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Parser, Resource, Debug)]
#[command(author, version, about, long_about = None)]
struct CommandLineArgs {
    /// Adds cheats to the game pause menu.
    #[arg(long, default_value_t = false)]
    cheat: bool,

    /// Prints the map
    #[arg(long, default_value_t = false)]
    verbose: bool,

    /// Select a specific seed
    #[arg(long)]
    map_seed: Option<u64>,
}

fn main() {
    println!("Cobblehelm Maze - Copyright (C) 2023 - Steven Wokke");

    let args = CommandLineArgs::parse();

    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it under certain conditions; type `show c' for details.");
    println!("");
    println!("Build: {}", env!("VERGEN_BUILD_DATE"));
    println!("git commit: {} ({})", env!("VERGEN_GIT_SHA"), env!("VERGEN_GIT_COMMIT_DATE"));

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cobblehem Maze".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(args)
        .add_state::<game::GameState>()
        .add_plugin(ui::UIPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(combat::CombatPlugin)
        .add_startup_system(app_setup)
        .run();
}

fn app_setup(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
    mut render_res: ResMut<rendering::SpriteResource>,
    mut commands: Commands,
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

    commands.spawn(Camera3dBundle {
        ..default()
    });
}

// This resource tracks the game's score
#[derive(Resource)]
pub struct GameInfo {
    pub hp_perc: f32,
    pub score: i32,
    pub coins: i32,
    pub level: u8,
    pub level_spawned: bool,
    pub difficulty: Difficulty,
    pub time: Stopwatch,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            hp_perc: 1.0,
            score: 0,
            coins: 0,
            level: 1,
            level_spawned: false,
            difficulty: Difficulty::Medium,
            time: Stopwatch::default(),
        }
    }
}