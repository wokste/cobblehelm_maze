mod combat;
mod game;
mod grid;
mod lifecycle;
mod map;
mod mapgen;
mod modelgen;
mod physics;
mod pickup;
mod rendering;
mod ui;

use bevy::{prelude::*, time::Stopwatch};

use clap::Parser;

#[derive(Parser, Resource, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Adds cheats to the game pause menu.
    #[arg(long, default_value_t = false)]
    cheat: bool,

    /// Prints the map
    #[arg(long, default_value_t = false)]
    verbose: bool,

    /// Prints the backtrace on an error
    #[arg(long, default_value_t = false)]
    trace: bool,

    /// Select a specific seed
    #[arg(long)]
    map_seed: Option<u64>,

    #[arg(long)]
    difficulty: Option<f32>,
}

fn main() {
    println!("Cobblehelm Maze - Copyright (C) 2023 - Steven Wokke");

    let args = CommandLineArgs::parse();

    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!(
        "This is free software, and you are welcome to redistribute it under certain conditions."
    );
    println!();
    println!("Build: {}", env!("VERGEN_BUILD_DATE"));
    println!(
        "git commit: {} ({})",
        env!("VERGEN_GIT_SHA"),
        env!("VERGEN_GIT_COMMIT_DATE")
    );

    if args.trace {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cobblehem Maze".into(),
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .insert_resource(args)
        .add_state::<game::GameState>()
        .add_plugins((ui::UIPlugin, game::GamePlugin, combat::CombatPlugin))
        .add_systems(Startup, app_setup)
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
        ..default() //Color::WHITE.into()
    });

    commands.spawn(Camera3dBundle { ..default() });
}

// This resource tracks the game's score
#[derive(Resource)]
pub struct GameInfo {
    pub hp_perc: f32,
    pub score: i32,
    pub coins: i32,
    pub level: u8,
    pub level_spawned: bool,
    pub time: Stopwatch,
    pub key_flags: u8,
}

impl GameInfo {
    fn update_hp(&mut self, stats: &combat::CreatureStats) {
        self.hp_perc = f32::clamp((stats.hp as f32) / (stats.hp_max as f32), 0.0, 1.0);
    }
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            hp_perc: 1.0,
            score: 0,
            coins: 0,
            level: 1,
            level_spawned: false,
            time: Stopwatch::default(),
            key_flags: 0,
        }
    }
}

#[derive(Resource)]
pub struct GameSettings {
    pub map_seed: Option<u64>,
    pub difficulty: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            map_seed: None,
            difficulty: 1.0,
        }
    }
}

impl GameSettings {
    pub fn from_daily(now: std::time::SystemTime) -> Self {
        let elapsed = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let elapsed_days = elapsed.as_secs() / 60 / 60 / 24;
        let mut seed = std::num::Wrapping(elapsed_days);

        // There is nothing magical about the numbers. These are merely used to avoid using seeds that people would randomly use.
        // Thanks to random.org
        seed ^= 465075828575581282;
        seed *= 765521045181377115;

        Self {
            map_seed: Some(seed.0),
            difficulty: 1.0,
        }
    }

    pub fn from_cl(args: &CommandLineArgs) -> Self {
        Self {
            map_seed: args.map_seed,
            difficulty: args.difficulty.unwrap_or(1.0),
        }
    }
}

impl GameInfo {
    fn next_level(&mut self) {
        self.level += 1;
        self.key_flags = 0;
        self.level_spawned = false;
    }
}
