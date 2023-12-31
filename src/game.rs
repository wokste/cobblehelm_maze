use bevy::{prelude::*, window::CursorGrabMode};

use crate::{
    combat::player::Player, lifecycle::LevelObject, map::MapData, mapgen::style::LevelStyle,
    spawner::Spawner,
};

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]

pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameMenu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::MainMenu), despawn_game)
            .add_systems(OnEnter(GameState::InGame), (start_level, capture_mouse))
            .add_systems(OnExit(GameState::InGame), release_mouse)
            .add_event::<crate::interactable::TriggerEvent>()
            .insert_resource(crate::map::MapData::default())
            .insert_resource(crate::render::RenderResource::default())
            .insert_resource(crate::GameInfo::default())
            .insert_resource(crate::GameSettings::default())
            .add_systems(
                Update,
                (
                    crate::physics::do_physics.after(crate::combat::player::handle_player_move),
                    crate::interactable::update_doors
                        .after(crate::combat::player::handle_player_interactions),
                    crate::items::pickup::check_pickups.after(crate::physics::do_physics),
                    crate::render::face_camera.after(crate::physics::do_physics),
                    crate::render::animate_sprites,
                    crate::lifecycle::check_ttl,
                )
                    .run_if(in_state(GameState::InGame)),
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
    render_res: ResMut<crate::render::RenderResource>,
    mut level_query: Query<Entity, With<LevelObject>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    game_settings: Res<crate::GameSettings>,
    cl_args: Res<crate::CommandLineArgs>,
) {
    if game_data.level_spawned {
        return; // No need to spawn the level again
    }
    game_data.level_spawned = true;

    if let Err(msg) = game_data.adjust_for_debug(&cl_args) {
        warn!("Could not set command line level settings. {}", msg);
    }

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
    let map_gen_result = crate::mapgen::make_map(level, game_data.level_style, &mut rng);
    if cl_args.verbose {
        crate::mapgen::print_map(&map_gen_result.tilemap);
    }

    let player_pos = Transform::from_translation(map_gen_result.player_pos.to_vec(0.7))
        .looking_to(Vec3::X, Vec3::Y);
    //    .looking_at(map_gen_result.spawn_objects[0].to_vec(0.7), Vec3::Y);
    *map_data = MapData {
        solid_map: map_gen_result.tilemap.map(|t| t.is_solid()),
        los_map: map_gen_result.tilemap.map(|t| t.is_solid()),
        monster_map: map_gen_result.tilemap.map(|t| t.is_solid()),
        player_pos,
        tile_map: map_gen_result.tilemap.clone(),
    };

    // Spawn the map mesh
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(crate::render::modelgen::map_to_mesh(
                &map_gen_result.tilemap,
                &render_res.sprites,
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
        let player = commands
            .spawn(crate::combat::player::PlayerBundle::new(
                map_gen_result.player_pos.to_vec(0.7),
            ))
            .insert(PbrBundle {
                transform: player_pos,
                ..default()
            })
            .id();

        assert_eq!(game_data.player, None);
        game_data.player = Some(player);
    }

    // Add the monsters
    let level_style = game_data.level_style;

    let mut spawner = Spawner {
        map_data,
        commands,
        meshes,
        render_res,
    };

    let monster_count = level * 3 + 12;
    for _ in 0..monster_count {
        use crate::mapgen::randitem::RandItem;
        let monster_type = *level_style.monsters().rand_front_loaded(&mut rng);
        spawner.try_spawn_monster(monster_type, &mut rng);
    }

    // Add level portal or phylactery
    for (pos, object_type) in map_gen_result.spawn_objects.iter() {
        spawner.spawn_object_at_pos(*pos, object_type, &mut rng);
    }

    // Add pickups
    {
        use crate::items::pickup::Pickup::*;
        for _ in 0..(level + 1) {
            spawner.try_spawn_item(Apple, &mut rng);
        }
        spawner.try_spawn_item(MedPack, &mut rng);

        let mut coins = get_coin_count(level, game_data.level_style);
        while coins > 0 {
            let (item, value) = if coins > 5 { (Gem, 5) } else { (Coin, 1) };

            spawner.try_spawn_item(item, &mut rng);
            coins -= value;
        }
    }

    // Add key pickups
    {
        use crate::items::pickup::Pickup as K;
        let mut keys = [K::Key(0), K::Key(1), K::Key(2), K::Key(3)];
        rng.shuffle(&mut keys);

        for key in keys.iter().take(2) {
            match spawner.choose_item_pos(&mut rng) {
                Ok(pos) => spawner.spawn_item_at_pos(pos, *key),
                Err(err) => println!("Failed top spawn item: {}", err),
            }
        }
    }
}

fn get_coin_count(level: u8, level_style: LevelStyle) -> i32 {
    let level_mult = (level + 1) as i32;
    let style_mult = match level_style {
        LevelStyle::Castle => 2,
        LevelStyle::Caves => 4,
        LevelStyle::Sewers => 6,
        LevelStyle::Hell => 8,
        LevelStyle::Machine => 10,
        LevelStyle::Ice => 7,
    };
    level_mult * style_mult
}
