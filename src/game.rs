use bevy::prelude::*;

use crate::{map::MapData, player::PlayerKeys};

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]

pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
    Paused,
    NextLevel
//    VendingMachine,
}

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system(despawn_game.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(start_level.in_schedule(OnEnter(GameState::InGame)))
            .insert_resource(crate::map::MapData::default())
            .insert_resource(crate::rendering::SpriteResource::default())
            .insert_resource(crate::GameInfo::default())
            .add_system(crate::player::player_input.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::player::update_map.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::ai::ai_los.in_set(OnUpdate(GameState::InGame)).after(crate::player::update_map))
            .add_system(crate::ai::ai_fire.in_set(OnUpdate(GameState::InGame)).after(crate::ai::ai_los))
            .add_system(crate::physics::do_physics.in_set(OnUpdate(GameState::InGame)).after(crate::player::player_input))
            .add_system(crate::weapon::check_projectile_creature_collisions.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::weapon::fire_weapons.in_set(OnUpdate(GameState::InGame)).after(crate::player::player_input).after(crate::ai::ai_fire))
    
            .add_system(crate::rendering::face_camera.in_set(OnUpdate(GameState::InGame)).after(crate::physics::do_physics))
            .add_system(crate::rendering::animate_sprites.in_set(OnUpdate(GameState::InGame)))
            ;
    }
}

/// set up the level
fn despawn_game(
    mut commands: Commands,
    mut map_data: ResMut<MapData>,
    mut level_query: Query<Entity, With<crate::LevelObject>>,
    mut player_query: Query<Entity, With<crate::player::PlayerKeys>>,
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
fn start_level(
    mut commands: Commands,
    game_data: Res<crate::GameInfo>,
    mut map_data: ResMut<MapData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<crate::rendering::SpriteResource>,
    mut level_query: Query<Entity, With<crate::LevelObject>>,
    mut player_query: Query<&mut Transform, With<PlayerKeys>>,
    cl_args: Res<crate::CommandLineArgs>,
) {
    if game_data.level_spawned {
        return; // No need to spawn the level again
    }

    for entity in level_query.iter_mut() {
        commands.entity(entity).despawn();
    }

    let mut rng = fastrand::Rng::new();
    if let Some(seed) = cl_args.map_seed { rng.seed(seed); }

    let level = game_data.level;
    println!("Seed: {}", rng.get_seed());

    // Get initial data
    let data = crate::procgen::make_map(level, &mut rng);
    if cl_args.verbose {
        crate::procgen::print_map(&data.map);
    }

    let player_pos = Transform::from_translation(data.player_pos.to_vec(0.7)).looking_at(data.stair_pos.to_vec(0.7), Vec3::Y);
    map_data.map = data.map;
    map_data.player_pos = player_pos.translation;

    // Spawn the map mesh
    commands.spawn(PbrBundle {
        mesh: meshes.add( crate::modelgen::map_to_mesh(&map_data.map, &mut rng)),
        material: render_res.material.clone(),
        ..default()
    }).insert(super::LevelObject);

    // Place the player in the map
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        *player_transform = player_pos.clone();
    } else {
        commands.spawn(crate::player::PlayerBundle::default()).insert(PbrBundle{
            transform: player_pos.clone(),
            ..default()
        });
    }

    // Add the monsters
    let level_style = crate::procgen::style::make_by_level(level);
    let monster_count = level * 5 + 15;
    for _ in 1 .. monster_count {
        use crate::procgen::randitem::RandItem;
        let monster_type = level_style.monsters.rand_front_loaded(&mut rng);
        let err = crate::ai::spawn_monster(&mut commands, &mut map_data, *monster_type, &mut meshes, &mut render_res, &mut rng);
        if let Err(err) = err {
            println!("Failed top spawn monster: {}", err);
        }
    }
}