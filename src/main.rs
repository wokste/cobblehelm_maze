mod combat;
mod game;
mod grid;
mod items;
mod lifecycle;
mod map;
mod mapgen;
mod physics;
mod render;
mod spawner;
mod spawnobject;
mod ui;
mod utils;

use bevy::{prelude::*, time::Stopwatch};

use clap::Parser;
use mapgen::style::LevelStyle;

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
    seed: Option<u64>,

    /// Format used is '<level>:<style>'
    #[arg(long)]
    level: Option<String>,

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
        .add_systems(Update, make_tileset_async)
        .run();
}

fn app_setup(
    asset_server: Res<AssetServer>,
    mut ambient_light: ResMut<AmbientLight>,
    mut commands: Commands,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.5;

    let mut builder = render::spritemapbuilder::SpriteMapBuilder::new();
    builder
        .start_load(&asset_server)
        .expect("Loading tilemap failed");
    commands.insert_resource(builder);

    commands.spawn(Camera3dBundle { ..default() });
}

fn make_tileset_async(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut render_res: ResMut<render::RenderResource>,
    mut images: ResMut<Assets<Image>>,
    mut builder: ResMut<render::spritemapbuilder::SpriteMapBuilder>,
) {
    if !builder.should_build(&images) {
        return;
    }

    render_res.sprites = builder.build(&mut images).expect("");

    render_res.material = materials.add(StandardMaterial {
        base_color_texture: Some(render_res.sprites.texture.clone()),
        alpha_mode: AlphaMode::Mask(0.5),
        unlit: true,
        ..default() //Color::WHITE.into()
    });
}

// This resource tracks the game's score
#[derive(Resource)]
pub struct GameInfo {
    pub player: Option<Entity>,
    pub score: i32,
    pub coins: i32,
    pub level: u8,
    pub level_style: LevelStyle,
    pub level_spawned: bool,
    pub time: Stopwatch,
    pub key_flags: u8,
    pub cheater: bool,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            player: None,
            score: 0,
            coins: 0,
            level: 1,
            level_style: LevelStyle::Castle,
            level_spawned: false,
            time: Stopwatch::default(),
            key_flags: 0,
            cheater: false,
        }
    }
}

impl GameInfo {
    pub fn adjust_for_debug(&mut self, args: &CommandLineArgs) -> Result<(), String> {
        let Some(level_str) = &args.level else {return Ok(());};
        let split: tinyvec::TinyVec<[&str; 2]> = level_str.split(':').collect();

        if split.len() != 2 {
            return Err(format!(
                "Level `{}` does not have the correct format.",
                level_str
            ));
        }

        let Ok(level) = split[0]
            .parse::<u8>() else {return Err("Not an int".to_string());};
        let level_style = crate::mapgen::style::LevelStyle::from_str(split[1])?;

        if !(1..=5).contains(&level) {
            return Err(format!("Level {} not in range", level));
        }
        self.level = level;
        self.level_style = level_style;
        self.cheater = true;

        Ok(())
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
            map_seed: args.seed,
            difficulty: args.difficulty.unwrap_or(1.0),
        }
    }
}

impl GameInfo {
    fn next_level(&mut self, level_index: LevelStyle) {
        self.level += 1;
        self.level_style = level_index;
        self.key_flags = 0;
        self.level_spawned = false;
    }
}
