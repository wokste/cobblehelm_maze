mod ai;
mod map;
mod mapgen;
mod modelgen;
mod physics;
mod player;
mod weapon;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) 
        .add_startup_system(setup)
        .insert_resource(map::MapData::default())
        .add_system(player::player_input)
        .add_system(ai::ai_los)
        .add_system(ai::ai_fire)
        .add_system(physics::do_physics)
        .add_system(player::update_map)
        .add_system(weapon::check_projectile_creature_collisions)
        .add_system(weapon::fire_weapons)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_data: ResMut<map::MapData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.5;

    let texture_handle = asset_server.load("C:/Users/wokste/Desktop/labyrinth_textures2.png");

    map_data.map = mapgen::make_map(fastrand::u8(1..=5));

    // The actual map
    commands.spawn(PbrBundle {
        mesh: meshes.add( modelgen::map_to_mesh(&map_data.map)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Opaque,
            unlit: true,
            ..default()
            //Color::WHITE.into()
        }),
        ..default()
    });

    // Player
    let player_pos = map_data.map.random_square();
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(player_pos.x as f32 + 0.5, 0.7, player_pos.z as f32 + 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(player::PlayerBundle::default());

    for _ in 1 .. 20 {
        ai::spawn_monster(&mut commands, &map_data, &mut meshes, &mut materials);
    }
}