use bevy::{prelude::*, window::CursorGrabMode};

use crate::{combat::player::Player, lifecycle::LevelObject, map::MapData};

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]

pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
    Paused,
    NextLevel,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(despawn_game.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(start_level.in_schedule(OnEnter(GameState::InGame)))
            .add_system(capture_mouse.in_schedule(OnEnter(GameState::InGame)))
            .add_system(release_mouse.in_schedule(OnExit(GameState::InGame)))
            .insert_resource(crate::map::MapData::default())
            .insert_resource(crate::rendering::SpriteResource::default())
            .insert_resource(crate::GameInfo::default())
            .insert_resource(crate::GameSettings::default())
            .add_systems(
                (
                    crate::physics::do_physics.after(crate::combat::player::get_player_input),
                    crate::pickup::check_pickups.after(crate::physics::do_physics),
                    crate::rendering::face_camera.after(crate::physics::do_physics),
                    crate::rendering::animate_sprites,
                    crate::lifecycle::check_ttl,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}

fn capture_mouse(mut windows: Query<&mut Window>) {
    for mut window in &mut windows {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}

fn release_mouse(mut windows: Query<&mut Window>) {
    for mut window in &mut windows {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

/// set up the level
fn despawn_game(
    mut commands: Commands,
    mut map_data: ResMut<MapData>,
    mut level_query: Query<Entity, With<LevelObject>>,
    mut player_query: Query<Entity, With<crate::combat::player::Player>>,
) {
    *map_data = MapData::default();

    for entity in level_query.iter_mut() {
        commands.entity(entity).despawn();
    }
    for entity in player_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

/// set up the level
#[allow(clippy::too_many_arguments)] // Not really applicable for bevy systems
fn start_level(
    mut commands: Commands,
    mut game_data: ResMut<crate::GameInfo>,
    mut map_data: ResMut<MapData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<crate::rendering::SpriteResource>,
    mut level_query: Query<Entity, With<LevelObject>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    game_settings: Res<crate::GameSettings>,
    cl_args: Res<crate::CommandLineArgs>,
) {
    if game_data.level_spawned {
        return; // No need to spawn the level again
    }
    game_data.level_spawned = true;

    for entity in level_query.iter_mut() {
        commands.entity(entity).despawn();
    }

    let mut rng = fastrand::Rng::new();
    if let Some(seed) = game_settings.map_seed {
        rng.seed(seed);
    }

    let level = game_data.level;
    println!("Seed: {}", rng.get_seed());

    // Get initial data
    let map_gen_result = crate::mapgen::make_map(level, &mut rng);
    if cl_args.verbose {
        crate::mapgen::print_map(&map_gen_result.tilemap);
    }

    let player_pos = Transform::from_translation(map_gen_result.player_pos.to_vec(0.7))
        .looking_at(map_gen_result.stair_pos.to_vec(0.7), Vec3::Y);
    map_data.solid_map = map_gen_result.tilemap.map(|t| t.is_solid());
    map_data.monster_map = map_gen_result.tilemap.map(|t| t.is_solid());
    map_data.los_map = map_gen_result.tilemap.map(|t| t.is_solid());
    map_data.player_pos = player_pos;

    // Spawn the map mesh
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(crate::modelgen::map_to_mesh(
                &map_gen_result.tilemap,
                &mut rng,
            )),
            material: render_res.material.clone(),
            ..default()
        })
        .insert(LevelObject);

    // Place the player in the map
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        *player_transform = player_pos;
    } else {
        commands
            .spawn(crate::combat::player::PlayerBundle::default())
            .insert(PbrBundle {
                transform: player_pos,
                ..default()
            });
    }

    // Add the monsters
    let level_style = crate::mapgen::style::make_by_level(level);
    let monster_count = level * 5 + 15;
    for _ in 0..monster_count {
        use crate::mapgen::randitem::RandItem;
        let monster_type = level_style.monsters.rand_front_loaded(&mut rng);
        let err = monster_type.spawn(
            &mut commands,
            &mut map_data,
            &mut meshes,
            &mut render_res,
            &mut rng,
        );
        if let Err(err) = err {
            println!("Failed top spawn monster: {}", err);
        }
    }

    // Add stairs
    crate::pickup::Pickup::NextLevel.spawn_at_pos(
        map_gen_result.stair_pos,
        &mut commands,
        &mut meshes,
        &mut render_res,
    );

    // Add pickups
    {
        let level = level as i32;
        use crate::pickup::Pickup::*;
        for (item_type, count) in [
            (Apple, 5),
            (MedPack, 1),
            (Coin, level + 5),
            (CoinPile, level * (level - 1) / 2),
        ] {
            for _ in 0..count {
                let err = item_type.spawn(
                    &mut commands,
                    &map_data,
                    &mut meshes,
                    &mut render_res,
                    &mut rng,
                );
                if let Err(err) = err {
                    println!("Failed top spawn item: {}", err);
                }
            }
        }
    }

    // Add key pickups
    {
        use crate::pickup::Pickup as K;
        let mut keys = [K::SilverKey, K::GoldKey, K::RedKey, K::GreenKey];
        rng.shuffle(&mut keys);

        for key in keys.iter().take(2) {
            let err = key.spawn(
                &mut commands,
                &map_data,
                &mut meshes,
                &mut render_res,
                &mut rng,
            );
            if let Err(err) = err {
                println!("Failed top spawn item: {}", err);
            }
        }
    }
}
